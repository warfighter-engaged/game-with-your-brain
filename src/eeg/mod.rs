//! This module handles connecting to and reading from a Neurosky EEG headset
//! using a UART connection.
//! 
//! Code adapted from <https://github.com/redpaperheart/ArduinoMindwave>
//! 
//! When the Mindwave struct goes out of scope, all relevant pins are reset to their original state.
//! This doesn't occur if an interrupt happens (ie Ctrl-C) unless a crate like `simple-signal` is used
//! to intercept these signals.

use crate::Result;

use std::time::Instant;
use std::time::Duration;
use rppal::uart::{Parity, Uart};

const BAUDRATE: u32 = 57_600;

pub struct Mindwave {
    debug: bool,
    new_packet: bool,
    payload_data: [u8; 64],
    poor_quality: u8,
    attention: u8,
    meditation: u8,
    last_received_packet: Instant,
    timeout: Duration,

    uart: Uart,
}

impl Mindwave {
    /// Initialize the Mindwave interface and opens a serial port at 57600 bauds.
    pub fn init() -> Result<Self> {
        // Connect to the primary UART and configure it according to the Arduino defaults:
        // 8 data bits, no parity, 1 stop bit (https://www.arduino.cc/reference/en/language/functions/communication/serial/begin/)
        let mut uart = Uart::new(BAUDRATE, Parity::None, 8, 1)?;

        // Flush the input
        let input_len = uart.input_len()?;
        for _ in 0..input_len {
            let mut buffer = [0u8; 1];
            uart.read(&mut buffer)?;
        }

        Ok(Self {
            debug: false,
            new_packet: false,
            payload_data: [0; 64],
            poor_quality: 250,
            attention: 0,
            meditation: 0,
            last_received_packet: Instant::now(),
            timeout: Duration::from_secs(5),

            uart,
        })
    }

    /// Listens for new brainwave data and parses it.
    pub fn update(&mut self) -> Result<()> {
        self.new_packet = false;

        // Look for sync bytes
        if self.read_first_byte()? != 0xAA {
            return Ok(());
        }
        if self.read_one_byte()? != 0xAA {
            return Ok(());
        }

        let payload_length = self.read_one_byte()?;
        if payload_length > 169 { // payload length cannot be greater than 169
            println!("!!! Payload length exceeded 169 bytes !!!");
            return Ok(()); // TODO: Return err
        }

        use std::num::Wrapping;

        let mut generated_checksum: Wrapping<u8> = Wrapping(0);
        for i in 0..payload_length {
            self.payload_data[i as usize] = self.read_one_byte()?;
            generated_checksum += Wrapping(self.payload_data[i as usize]);
        }

        let checksum = self.read_one_byte()?;
        generated_checksum = Wrapping(0xFF) - generated_checksum; // Take one's complement of generated checksum

        if checksum != generated_checksum.0 {
            println!("!!! CHECKSUM ERROR !!!");
            return Ok(()); // TODO: Return err
        }

        self.poor_quality = 200;
        self.attention = 0;
        self.meditation = 0;

        let mut i = 0;
        while i < (payload_length as usize) {
            match self.payload_data[i] {
                2 => {
                    i += 1;
                    self.poor_quality = self.payload_data[i];
                    self.new_packet = true;
                }
                4 => {
                    i += 1;
                    self.attention = self.payload_data[i];
                }
                5 => {
                    i += 1;
                    self.meditation = self.payload_data[i];
                }
                0x80 => {
                    i += 3;
                }
                0x83 => {
                    i += 25;
                }
                _ => {}
            }
            i += 1;
        }

        let now = Instant::now();
        let dur = now - self.last_received_packet;

        if self.new_packet {
            if self.debug {
                print!("PoorQuality: {}", self.poor_quality);
                print!(" Attention: {}", self.attention);
                print!(" Meditation: {}", self.meditation);
                println!(" Time since last packet: {}", dur.as_millis() as f32 / 1000f32);
            }

            self.last_received_packet = now;
        } else if dur > self.timeout {
            if self.poor_quality != 200 {
                println!("!!! Poor Quality - Check Connection !!!");
            }
            self.poor_quality = 200;
            self.attention = 0;
            self.meditation = 0;
        }

        Ok(())
    }

    /// Tells the Mindwave class whether to print the received data or not.
    #[inline]
    pub fn set_debug(&mut self, d: bool) {
        self.debug = d;
    }
    
    /// Sets the duration that it will take before deciding that there is not data coming in, and setting the quality to 0. Default is 5000 (5 seconds).
    #[inline]
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Returns a boolean indicating if a new data packet has been parsed.
    #[inline]
    pub fn has_new_data(&self) -> bool {
        self.new_packet
    }

    /// Returns a boolean indicating if the Mindwave is set to debug.
    #[inline]
    pub fn is_debugging(&self) -> bool {
        self.debug
    }

    /// Returns a number from 0 (bad) to 100 (good) with the level of attention.
    #[inline]
    pub fn get_attention(&self) -> u8 {
        self.attention
    }

    /// Returns a number from 0 (bad) to 100 (good) with the level of meditation.
    #[inline]
    pub fn get_meditation(&self) -> u8 {
        self.meditation
    }

    /// Returns a number from 0 to 200 with the quality of the signal. Quality goes from 0 (good quality) to 200 (bad).
    #[inline]
    pub fn get_poor_quality(&self) -> u8 {
        self.poor_quality
    }

    /// Returns a number from 0 to 200 with the quality of the signal. Quality goes from 0 (bad quality) to 200 (good).
    #[inline]
    pub fn get_quality(&self) -> u8 {
        200 - self.poor_quality
    }

    /// Read data from serial UART (non-blocking).
    /// If no data is available, returns 0
    fn read_first_byte(&mut self) -> Result<u8> {
        // Set the UART to read in non-blocking mode (0 minimum length, 0 timeout)
        self.uart.set_read_mode(0, Duration::default())?;

        let mut buffer = [0u8; 1];
        if self.uart.read(&mut buffer)? > 0 {
            Ok(buffer[0])
        } else {
            Ok(0)
        }
    }

    /// Read data from serial UART (blocking)
    fn read_one_byte(&mut self) -> Result<u8> {
        self.uart.set_read_mode(1, Duration::default())?; // TODO: determine appropriate timeout?
        let mut buffer = [0u8; 1];
        if self.uart.read(&mut buffer)? > 0 {
            Ok(buffer[0])
        } else {
            println!("!!!ERROR!!! read_one_byte did not read a byte");
            Ok(0)
        }
    }
}