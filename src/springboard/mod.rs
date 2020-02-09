//! This module handles connecting to and sending data to an XBOX Adaptive Controller.
//! To do this, we leverage the Raspberry Pi's PWM pins.
//!
//! Since the Pi's analog audio output uses both PWM channels, using them and playing
//! audio simultaneously may cause issues.

cfg_if::cfg_if! {
    if #[cfg(feature = "adafruit")] {
        mod adafruit3502;
        use rppal::i2c;
    } else {
        mod mcp4922;
        use rppal::spi;
    }
}

use crate::Result;

use rppal::gpio;

const GPIO_LEFT_BTN: u8 = 23; // BCM GPIO 23 is tied to phyiscal pin 16
const GPIO_RIGHT_BTN: u8 = 22; // BCM GPIO 22 is tied to physical pin 15

const I2C_TRIGGER_BUS: u8 = 1; // For the early model B Rev 1, bus 0 is selected. For every other model, bus 1 is used.
                               // This is tied to physical pin 3 and 5 (SDA and SCL)

const SPI_MAX_CLOCK_SPEED: u32 = 1000;

cfg_if::cfg_if! {
    if #[cfg(feature = "adafruit")] {
        pub struct Springboard {
            left_btn: gpio::OutputPin,
            right_btn: gpio::OutputPin,
            trigger: adafruit3502::AdafruitDS3502,
        }
    } else {
        pub struct Springboard {
            left_btn: gpio::OutputPin,
            right_btn: gpio::OutputPin,
            trigger: mcp4922::Mcp4922,
        }
    }
}

impl Springboard {
    pub fn init() -> Result<Self> {
        let gpio_ = gpio::Gpio::new()?;
        let left_btn = gpio_.get(GPIO_LEFT_BTN)?.into_output();
        let right_btn = gpio_.get(GPIO_RIGHT_BTN)?.into_output();

        cfg_if::cfg_if! {
            if #[cfg(feature = "adafruit")] {
                let trigger_bus = i2c::I2c::with_bus(I2C_TRIGGER_BUS)?;
                let mut trigger = adafruit3502::AdafruitDS3502::new(trigger_bus);
                trigger.begin(adafruit3502::DS3502_I2CADDR_DEFAULT)?;
            } else {
                let trigger_bus = spi::Spi::new(
                    spi::Bus::Spi0,
                    spi::SlaveSelect::Ss1,
                    SPI_MAX_CLOCK_SPEED,
                    spi::Mode::Mode0,
                )?;
                let trigger = mcp4922::Mcp4922::new(trigger_bus);
            }
        }

        Ok(Self {
            left_btn,
            right_btn,
            trigger,
        })
    }

    pub fn update_left_btn(&mut self, pressed: bool) {
        self.left_btn.write(if pressed {
            gpio::Level::High
        } else {
            gpio::Level::Low
        });
    }

    pub fn update_right_btn(&mut self, pressed: bool) {
        self.right_btn.write(if pressed {
            gpio::Level::High
        } else {
            gpio::Level::Low
        });
    }

    /// Update the trigger pull to a value in the range [0, 100]
    pub fn update_trigger(&mut self, value: f64) -> Result<()> {

        cfg_if::cfg_if! {
            if #[cfg(feature = "adafruit")] {
                // The wiper expects a value in the range [0, 127]
                self.trigger.set_wiper((value * 127f64 / 100f64) as u8)?;
            } else {
                // The wiper expects a value in the range [0, 4095]
                self.trigger
                    .set_wiper(mcp4922::Channel::CHA, (value * 4095f64 / 100f64) as u16)?;
            }
        }
        
        Ok(())
    }
}
