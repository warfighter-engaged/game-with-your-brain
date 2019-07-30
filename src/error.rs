use failure::Fail;

#[derive(Debug, Fail)]
pub enum WfpiError {
    #[fail(display = "error in eeg: {}", err)]
    EegError {
        err: rppal::uart::Error,
    },
    #[fail(display = "raspberry pi system error: {}", err)]
    SystemError {
        err: rppal::system::Error,
    },
    #[fail(display = "gpio error: {}", err)]
    GpioError {
        err: rppal::gpio::Error,
    },
    #[fail(display = "threading error: {}", err)]
    ThreadingError {
        err: std::sync::mpsc::RecvError,
    }
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

pub type Result<T> = std::result::Result<T, WfpiError>;