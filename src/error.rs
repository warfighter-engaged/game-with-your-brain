use failure::Fail;

#[derive(Debug, Fail)]
pub enum WfpiError {
    #[fail(display = "error in eeg: {}", err)]
    EegError { err: rppal::uart::Error },
    #[fail(display = "raspberry pi system error: {}", err)]
    SystemError { err: rppal::system::Error },
    #[fail(display = "gpio error: {}", err)]
    GpioError { err: rppal::gpio::Error },
    #[fail(display = "threading error: {}", err)]
    ThreadingError { err: std::sync::mpsc::RecvError },
    #[fail(display = "spi error: {}", err)]
    SpiError { err: rppal::spi::Error },
    #[fail(display = "i2c error: {}", err)]
    I2CErr { err: rppal::i2c::Error },
    #[fail(display = "i/o error: {}", err)]
    IoError { err: std::io::Error },
    #[fail(display = "generic error: {}", err)]
    GenericError { err: failure::Error },
}

impl From<rppal::uart::Error> for WfpiError {
    fn from(err: rppal::uart::Error) -> WfpiError {
        WfpiError::EegError { err }
    }
}

impl From<rppal::system::Error> for WfpiError {
    fn from(err: rppal::system::Error) -> WfpiError {
        WfpiError::SystemError { err }
    }
}

impl From<rppal::gpio::Error> for WfpiError {
    fn from(err: rppal::gpio::Error) -> WfpiError {
        WfpiError::GpioError { err }
    }
}

impl From<std::sync::mpsc::RecvError> for WfpiError {
    fn from(err: std::sync::mpsc::RecvError) -> WfpiError {
        WfpiError::ThreadingError { err }
    }
}

impl From<rppal::spi::Error> for WfpiError {
    fn from(err: rppal::spi::Error) -> WfpiError {
        WfpiError::SpiError { err }
    }
}

impl From<rppal::i2c::Error> for WfpiError {
    fn from(err: rppal::i2c::Error) -> WfpiError {
        WfpiError::I2CErr { err }
    }
}

impl From<std::io::Error> for WfpiError {
    fn from(err: std::io::Error) -> WfpiError {
        WfpiError::IoError { err }
    }
}

impl From<failure::Error> for WfpiError {
    fn from(err: failure::Error) -> WfpiError {
        WfpiError::GenericError { err }
    }
}

pub type Result<T> = std::result::Result<T, WfpiError>;
