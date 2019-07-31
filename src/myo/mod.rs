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
//! Here we assume that the left MYO sensor

use crate::Result;

use rppal::spi;

const SPI_BUS: spi::Bus = spi::Bus::Spi0;
const SPI_SLAVE_SELECT: spi::SlaveSelect = spi::SlaveSelect::Ss0;
const SPI_MAX_CLOCK_SPEED: u32 = 9600; // This approximates the Arduino ADC default sample rate
const SPI_MODE: spi::Mode = spi::Mode::Mode0;

#[derive(Debug, Clone)]
pub enum Side {
    Left = 0,
    Right = 1,
}

pub struct MyoReader {
    new_data: bool,
    values: [u16; 2],

    spi: spi::Spi,
}

impl MyoReader {
    pub fn init() -> Result<Self> {
        let spi = spi::Spi::new(SPI_BUS, SPI_SLAVE_SELECT, SPI_MAX_CLOCK_SPEED, SPI_MODE)?;
        Ok(Self {
            new_data: false,
            values: [0u16; 2],
            spi,
        })
    }

    fn update_channel(&mut self, channel: u8) -> Result<()> {
        // conversion is configured for single-ended conversion on the specified channel.
        // for example:
        // transmit -> byte1 = 0b0000_0001 (start bit)
        //             byte2 = 0b1000_0000 (SGL/DIF = 1, D2=D1=D0 = 0)
        //             byte3 = 0b0000_0000 (don't care)
        // receive  -> byte1 = junk
        //             byte2 = junk + b8 + b9
        //             byte3 = b7 - b0
        // after conversion merge read_buffer[1] and read_buffer[2] to get final result

        let mut write_buffer: [u8; 3] = [0u8; 3];

        write_buffer[0] = 1; // first byte transmitted -> start bit
        write_buffer[1] = 0b1000_0000 | ((channel & 7) << 4); // second byte transmitted -> (SGL/DIF = 1, D2,D1,D0 = channel)
        write_buffer[2] = 0; // third byte transmitted...don't care

        let mut read_buffer: [u8; 3] = [0u8; 3];

        self.spi.transfer(&mut read_buffer, &write_buffer)?;

        let mut adc_val: u16;
        adc_val = (u16::from(read_buffer[1]) << 8) & 0b11_0000_0000; // merge read_buffer[1] and read_buffer[2] to get result
        adc_val |= u16::from(read_buffer[2]) & 0xff;

        if self.values[channel as usize] != adc_val {
            self.new_data = true;
            self.values[channel as usize] = adc_val;
        }

        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        self.new_data = false;

        self.update_channel(Side::Left as u8)?;
        self.update_channel(Side::Right as u8)?;

        Ok(())
    }

    pub fn has_new_data(&self) -> bool {
        self.new_data
    }

    pub fn get_value(&self, side: Side) -> u16 {
        self.values[side as usize]
    }
}