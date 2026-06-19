mod state_machine;

use embedded_hal::delay::DelayNs;
use embedded_io::Read;

use crate::camsense_x1::state_machine::StateMachineWrapper;
use crate::constants::NUMBER_OF_MEASUREMENTS;
use crate::types::{Error, RawMeasurement};
use crate::{Measurement, Scan};

/// Camsense-X1 controller configuration
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    angle_offset: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self { angle_offset: 13.0 }
    }
}

/// Camsense-X1 controller
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Camsense<UART: Read, D: DelayNs> {
    /// Concrete UART implementation.
    uart: UART,
    delay: D,
    state_machine: StateMachineWrapper,
    config: Config,
}

impl<UART, D> Camsense<UART, D>
where
    UART: Read,
    D: DelayNs,
{
    pub fn new(uart: UART, delay: D) -> Self {
        Self::with_config(uart, delay, Config::default())
    }

    pub fn with_config(uart: UART, delay: D, config: Config) -> Self {
        Self {
            uart,
            delay,
            state_machine: StateMachineWrapper::new(),
            config,
        }
    }

    pub fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N], Error<UART::Error>> {
        let mut buffer = [0; N];
        self.uart.read(&mut buffer).map_err(Error::UART)?;
        Ok(buffer)
    }

    pub fn read_sample(&mut self) -> Result<Measurement, Error<UART::Error>> {
        loop {
            let byte = self.read_bytes::<1>()?;
            self.state_machine = self.state_machine.step(byte[0]);

            if let Some(buf) = self.state_machine.take_buf() {
                let data = *buf;
                self.state_machine.reset();

                match RawMeasurement::try_from(data) {
                    Ok(raw) => return Ok((raw, self.config.angle_offset).into()),
                    Err(Error::ChecksumMismatch(_, _)) => {
                        // just continue the loop, state machine already reset
                    }
                    Err(_) => return Err(Error::Other),
                }
            }
        }
    }

    pub fn read_scan(&mut self) -> Result<Scan, Error<UART::Error>> {
        let mut points = [None; 400];
        for _ in 0..NUMBER_OF_MEASUREMENTS {
            let measurement = self.read_sample()?;
            for point in measurement.points {
                if let Some(point) = point {
                    points[point.index] = Some(point);
                }
            }
        }
        let point_cloud = Scan { points };
        Ok(point_cloud)
    }
}
