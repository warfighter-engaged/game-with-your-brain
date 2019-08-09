//! This module handles connecting to and sending data to an XBOX Adaptive Controller.
//! To do this, we leverage the Raspberry Pi's PWM pins.
//! 
//! Since the Pi's analog audio output uses both PWM channels, using them and playing
//! audio simultaneously may cause issues.

use crate::Result;

use rppal::pwm;
use rppal::gpio;

const GPIO_LEFT_BTN: u8 = 23; // BCM GPIO 23 is tied to phyiscal pin 16
const GPIO_RIGHT_BTN: u8 = 22; // BCM GPIO 22 is tied to physical pin 15

const PWM_TRIGGER_CHANNEL: pwm::Channel = pwm::Channel::Pwm0; // channel 0 is tied to BCM GPIO 12, physical pin 32
const PWM_REFERENCE_CHANNEL: pwm::Channel = pwm::Channel::Pwm1; // channel 1 is tied to BCM GPIO 13, physical pin 33

// const PWM_FREQUENCY: f64 = 490.0; // 490 Hz matches the frequency of the Arduino Mega analog pins used
const PWM_FREQUENCY: f64 = 1000.0;

pub struct Springboard {
    left_btn: gpio::OutputPin,
    right_btn: gpio::OutputPin,

    trigger: pwm::Pwm,
    trigger_ref: pwm::Pwm,
}

impl Springboard {
    pub fn init() -> Result<Self> {
        let gpio_ = gpio::Gpio::new()?;
        let left_btn = gpio_.get(GPIO_LEFT_BTN)?.into_output();
        let right_btn = gpio_.get(GPIO_RIGHT_BTN)?.into_output();

        let trigger = pwm::Pwm::with_frequency(PWM_TRIGGER_CHANNEL, PWM_FREQUENCY, 0.0, pwm::Polarity::Normal, true)?;
        let trigger_ref = pwm::Pwm::with_frequency(PWM_REFERENCE_CHANNEL, PWM_FREQUENCY, 0.0, pwm::Polarity::Normal, true)?;

        Ok(Self {
            left_btn,
            right_btn,
            trigger,
            trigger_ref,
        })
    }

    pub fn update_left_btn(&mut self, pressed: bool) {
        self.left_btn.write(if pressed { gpio::Level::High } else { gpio::Level::Low } );
    }

    pub fn update_right_btn(&mut self, pressed: bool) {
        self.right_btn.write(if pressed { gpio::Level::High } else { gpio::Level::Low } );
    }

    /// Update the trigger pull to a value
    pub fn update_trigger(&mut self, value: f64) -> Result<()> {
        self.trigger.set_duty_cycle(value / 100f64)?;
        self.trigger_ref.set_duty_cycle(0.0f64)?;
        Ok(())
    }
}