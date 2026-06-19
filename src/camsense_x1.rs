mod state_machine;

use embedded_hal::delay::DelayNs;
use embedded_io::Read;

use crate::camsense_x1::state_machine::StateMachineWrapper;
use crate::types::{Error, RawMeasurement};
use crate::{Measurement, PointCloud};

/// Camsense-X1 controller
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Camsense<UART: Read, D: DelayNs> {
    /// Concrete UART implementation.
    uart: UART,
    delay: D,
    state_machine: StateMachineWrapper,
}

impl<UART, D> Camsense<UART, D>
where
    UART: Read,
    D: DelayNs,
{
    pub fn new(uart: UART, delay: D) -> Self {
        let state_machine = StateMachineWrapper::new();
        Self {
            uart,
            delay,
            state_machine,
        }
    }

    pub fn read<const N: usize>(&mut self) -> Result<[u8; N], Error<UART::Error>> {
        let mut buffer = [0; N];
        self.uart.read(&mut buffer).map_err(Error::UART)?;
        Ok(buffer)
    }

    pub fn read_one_measurement(&mut self) -> Result<Measurement, Error<UART::Error>> {
        loop {
            let byte = self.read::<1>()?;
            self.state_machine = self.state_machine.step(byte[0]);

            if let Some(buf) = self.state_machine.take_buf() {
                let data = *buf;
                self.state_machine.reset();

                match RawMeasurement::try_from(data) {
                    Ok(raw) => return Ok(raw.into()),
                    Err(Error::ChecksumMismatch(e, c)) => {
                        // just continue the loop, state machine already reset
                    }
                    Err(_) => return Err(Error::Other),
                }
            }
        }
    }

    pub fn read_point_cloud(&mut self) -> Result<PointCloud, Error<UART::Error>> {
        let mut points = [None; 400];
        for i in 0..50 {
            let measurement = self.read_one_measurement()?;
            for point in measurement.points {
                if let Some(point) = point {
                    points[point.index] = Some(point);
                }
            }
        }
        let point_cloud = PointCloud { points };
        Ok(point_cloud)
    }
}
