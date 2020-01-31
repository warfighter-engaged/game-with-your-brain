use rppal::i2c;

use crate::error::*;

/// The default starting address
pub const DS3502_I2CADDR_DEFAULT: u8 = 0x28;

const DS3502_WIPER: u8 = 0x00;
const DS3502_MODE: u8 = 0x02;

pub struct AdafruitDS3502 {
    pub bus: i2c::I2c,
}

impl AdafruitDS3502 {
    pub fn new(bus: i2c::I2c) -> Self {
        Self { bus }
    }

    pub fn begin(&mut self, i2c_addr: u8) -> Result<()> {
        self.bus.set_slave_address(i2c_addr as u16)?;

        // Select mode
        self.bus.smbus_write_byte(DS3502_MODE, 0x80)?;

        Ok(())
    }

    pub fn wiper(&self) -> Result<u8> {
        let res = self.bus.smbus_read_byte(DS3502_WIPER)?;
        Ok(res)
    }

    pub fn set_wiper(&self, value: u8) -> Result<()> {
        if value > 127 {
            return Ok(()); // TODO: Should be error
        }
        self.bus.smbus_write_byte(DS3502_WIPER, value)?;
        Ok(())
    }

    pub fn set_wiper_default(&self, default: u8) -> Result<()> {
        // Set mode to write default on wiper write
        self.bus.smbus_write_byte(DS3502_MODE, 0x00)?;
        // Write the new default
        self.bus.smbus_write_byte(DS3502_WIPER, default)?;
        // TODO: delay to allow EEPROM write to IVR to finish? delay(100)

        // Set mode back to regular writes
        self.bus.smbus_write_byte(DS3502_MODE, 0x80)?;

        Ok(())
    }
}
