//! This module handles connecting to and reading from a MYO electric sensor.
//! It assumes the use of an MCP3008 analog-to-digital converter connected to
//! a raspberry pi via SPI.
//!
//! All 40-pin raspberry pi models provide two SPI buses: SPI0 and SPI1. SPI1 has
//! a few limitations, so we'll use SPI0. However, this bus must be enabled by running
//! `sudo raspi-config`.
//!
//! The associated pins are:
//! * MISO: BCM GPIO 9 (physical pin 21)
//! * MOSI: BCM GPIO 10 (physical pin 19)
//! * SCLK: BCM GPIO 11 (physical pin 23)
//! * SS: s0 BCM GPIO 8 (physical pin 24), Ss1 BCM GPIO 7 (physical pin 26)
//!
//! Here we assume that the left MYO sensor is attached to channel 0, and the right sensor is
//! attached to channel 1.

use crate::Result;

// use crate::emg_process::*;
use rppal::spi;

mod emg_filters;
mod flappy;

const SPI_BUS: spi::Bus = spi::Bus::Spi0;
const SPI_SLAVE_SELECT: spi::SlaveSelect = spi::SlaveSelect::Ss0;
const SPI_MAX_CLOCK_SPEED: u32 = 1000; // This approximates the Arduino ADC default sample rate
const SPI_MODE: spi::Mode = spi::Mode::Mode0;

#[derive(Debug, Clone)]
pub enum Side {
    Left = 0,
    Right = 1,
    Analog = 2,
}

pub struct MyoReader {
    new_data: bool,
    values: [u16; 3],

    spi: spi::Spi,
}

impl MyoReader {
    pub fn init() -> Result<Self> {
        let spi = spi::Spi::new(SPI_BUS, SPI_SLAVE_SELECT, SPI_MAX_CLOCK_SPEED, SPI_MODE)?;
        Ok(Self {
            new_data: false,
            values: [0u16; 3],
            spi,
        })
    }

    fn update_channel(&mut self, channel: u8, differential: bool) -> Result<()> {
        // conversion is configured for single-ended conversion on the specified channel.
        // for example:
        // transmit -> byte1 = 0b0000_0001 (start bit)
        //             byte2 = 0b1000_0000 (SGL/DIF = 1, D2=D1=D0 = 0)
        //             byte3 = 0b0000_0000 (don't care)
        // receive  -> byte1 = junk
        //             byte2 = junk + b8 + b9
        //             byte3 = b7 - b0
        // after conversion merge read_buffer[1] and read_buffer[2] to get final result

        let sgldiff = if differential { 0 } else { 1 };

        let command: u8 = (0x01 << 7) | (sgldiff << 6) | ((channel & 0x7) << 3);

        let tx_buf = [command, 0x0, 0x0];
        let mut rx_buf = [0_u8; 3];

        self.spi.transfer(&mut rx_buf, &tx_buf)?;

        let mut result = (rx_buf[0] as u16 & 0x01) << 9;
        result |= (rx_buf[1] as u16 & 0xFF) << 1;
        result |= (rx_buf[2] as u16 & 0x80) >> 7;
        result &= 0x3FF;

        if self.values[channel as usize] != result {
            self.new_data = true;
            self.values[channel as usize] = result;
        }

        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        self.new_data = false;

        self.update_channel(Side::Left as u8, false)?;
        self.update_channel(Side::Right as u8, false)?;

        Ok(())
    }

    pub fn has_new_data(&self) -> bool {
        self.new_data
    }

    pub fn get_value(&self, side: Side) -> u16 {
        self.values[side as usize]
    }
}

pub struct MyoParser {
    reader: MyoReader,
    left_emg: flappy::Flappy,
    right_emg: flappy::Flappy,

    left_val: i32,
    left_state: bool,
    right_val: i32,
    right_state: bool,

    analog_state: u16,
}

impl MyoParser {
    /// Creates a new MYO parser
    pub fn new() -> Result<Self> {
        Ok(Self {
            reader: MyoReader::init()?,
            left_emg: flappy::Flappy::new(),
            right_emg: flappy::Flappy::new(),
            left_val: 0,
            left_state: false,
            right_val: 0,
            right_state: false,

            analog_state: 0,
        })
    }

    /// Updates the MYOs, returns true if there's new data, false otherwise
    pub fn update(&mut self) -> Result<bool> {
        self.reader.update()?;
        let res = self.reader.has_new_data();

        if res {
            let (ls, lv) = self.left_emg.update(self.reader.get_value(Side::Left));
            let (rs, rv) = self.right_emg.update(self.reader.get_value(Side::Right));

            let analog_out = self.reader.get_value(Side::Analog);
            if analog_out != self.analog_state {
                log!("New analog reading: {}", analog_out);
                self.analog_state = analog_out;
            }

            self.left_val = lv;
            self.left_state = ls;
            self.right_val = rv;
            self.right_state = rs;
        }

        Ok(res)
    }

    /// Gets the SDFT result for the given side
    pub fn get_value(&self, side: Side) -> (bool, i32) {
        match side {
            Side::Left => (self.left_state, self.left_val),
            Side::Right => (self.right_state, self.right_val),
            Side::Analog => (false, 0),
        }
    }
}
