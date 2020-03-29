use rppal::i2c;

use crate::error::*;

/// The default starting address
pub const I2CADDR_DEFAULT: u8 = 0b0101111;

pub struct Mcp4018 {
    pub bus: i2c::I2c,
}

impl Mcp4018 {
    pub fn new(bus: i2c::I2c) -> Self {
        Self { bus }
    }

    pub fn begin(&mut self, i2c_addr: u8) -> Result<()> {
        self.bus.set_slave_address(i2c_addr as u16)?;

        Ok(())
    }

    pub fn wiper(&self) -> Result<u8> {
        let res = self.bus.smbus_receive_byte()?;
        Ok(res)
    }

    pub fn set_wiper(&self, value: u8) -> Result<()> {
        if value > 127 {
            return Ok(()); // TODO: Should be error
        }
        self.bus.smbus_send_byte(value)?;
        Ok(())
    }
}
