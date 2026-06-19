use embedded_io::{Error as EmbeddedErrorTrait, ErrorKind, ErrorType, Read};
use linux_embedded_hal::serialport::SerialPort;

#[derive(Debug)]
pub enum Error {
    Uart(std::io::Error),
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
    uart: Box<dyn SerialPort>,
}

impl WrappedUart {
    pub fn new(uart: Box<dyn SerialPort>) -> Self {
        Self { uart }
    }
}

impl ErrorType for WrappedUart {
    type Error = Error;
}

impl Read for WrappedUart {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.uart.read(buf).map_err(Error::Uart)
    }
}
