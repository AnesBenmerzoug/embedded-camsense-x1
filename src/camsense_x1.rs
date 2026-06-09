mod state_machine;

use embedded_io::{Read};
use embedded_hal::{
    delay::DelayNs,
};

use crate::types::{Error, RawMeasurement};
use crate::camsense_x1::state_machine::{StateMachineWrapper};
use crate::{Measurement, PointCloud};


/// Camsense-X1 controller
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Camsense<UART: Read, D: DelayNs> {
    /// Concrete UART implementation.
    uart: UART,
    delay: D,
}

impl<UART, D> Camsense<UART, D> 
where 
    UART: Read,
    D: DelayNs
{
    pub fn new(uart: UART, delay: D) -> Self {
        Self { uart, delay,  }
    }

    pub fn read<const N: usize>(&mut self) -> Result<[u8; N], Error<UART::Error>> {
        let mut buffer = [0; N];
        self.uart.read(&mut buffer).map_err(Error::UART)?;
        Ok(buffer)
    }

    pub fn read_one_measurement(&mut self) -> Result<Measurement, Error<UART::Error>> {
        let mut state_machine = StateMachineWrapper::new();
        loop {
            match state_machine {
                StateMachineWrapper::Data(data_state_machine) => {
                    let remaining_bytes = self.read::<31>()?;
                    let mut data = [0; 32];
                    data[0] = data_state_machine.first_byte();
                    data[1..].copy_from_slice(&remaining_bytes);
                    let raw_measurement = RawMeasurement::from(data);
                    let measurement: Measurement = raw_measurement.into();
                    return Ok(measurement);
                },
                _ => {
                    let byte = self.read::<1>()?;
                    state_machine = state_machine.step(byte[0]);
                }
            }
        }
    }

    pub fn read_point_cloud(&mut self) -> Result<PointCloud, Error<UART::Error>> {
        let mut points = [None; 400];
        for i in 0..50 {
            let measurement =self.read_one_measurement()?;
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