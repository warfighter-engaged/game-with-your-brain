use rppal::spi;

use crate::error::*;

pub struct Mcp4922 {
    spi: spi::Spi,
}

impl Mcp4922 {
    pub fn new(spi: spi::Spi) -> Self {
        Mcp4922 { spi }
    }

    pub fn set_wiper(&mut self, channel: Channel, value: u16) -> Result<()> {
        let channel_bit = match channel {
            Channel::CHA => 0,
            Channel::CHB => 1,
        };
        let buffer_bit = 0; // Unbuffered
        let output_gain_bit = 1; // 1x
        let power_down_bit = 0; // Not power down

        let config_bits =
            channel_bit << 3 | buffer_bit << 2 | output_gain_bit << 1 | power_down_bit;
        // Compose the first byte to send the DAC:
        // the 4 control bits, and the 4 most significant bits of the value
        let first_byte = config_bits << 4 | (value & 0xF00) >> 8;
        // Second byte is the lower 8 bits of the value
        let second_byte = value & 0xFF;

        let tx_buf = [first_byte as u8, second_byte as u8];
        self.spi.write(&tx_buf)?;

        Ok(())
    }

    pub fn shutdown(&mut self, channel: Channel) -> Result<()> {
        let channel_bit = match channel {
            Channel::CHA => 0,
            Channel::CHB => 1,
        };
        let buffer_bit = 0; // Unbuffered
        let output_gain_bit = 1; // 1x
        let power_down_bit = 1; // Not power down

        let config_bits =
            channel_bit << 3 | buffer_bit << 2 | output_gain_bit << 1 | power_down_bit;
        // Compose the first byte to send the DAC:
        // the 4 control bits, and the 4 most significant bits of the value
        let first_byte = config_bits << 4;
        // Second byte is the lower 8 bits of the value
        let second_byte = 0xFF;

        let tx_buf = [first_byte, second_byte];
        self.spi.write(&tx_buf)?;

        Ok(())
    }
}

pub enum Channel {
    CHA,
    CHB,
}
