use rppal::uart::{Uart, Error as RppalError};
use embedded_io::{ErrorType, Error as EmbeddedErrorTrait, ErrorKind, Read};

#[derive(Debug)]
pub enum Error {
    Uart(RppalError),
    Other,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl EmbeddedErrorTrait for Error {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

pub struct WrappedUart {
    uart: Uart
}

impl WrappedUart {
    pub fn new(uart: Uart) -> Self {
        Self { uart }
    }
}

impl ErrorType for WrappedUart {
    type Error = Error;
}

impl Read for WrappedUart {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.uart.read(buf).map_err(|x| Error::Uart(x))
    }
}
