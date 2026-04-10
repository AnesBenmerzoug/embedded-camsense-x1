mod state_machine;

use embedded_io::{Read};
use embedded_hal::{
    delay::DelayNs,
};

use crate::types::{Error, RawMeasurement};
use crate::camsense_x1::state_machine::StateMachineWrapper;
use crate::Measurement;


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

    pub fn get_point_cloud(&mut self) -> Result<(RawMeasurement, Measurement), Error<UART::Error>> {
        let mut state_machine = StateMachineWrapper::new();
        loop {
            println!("state_machine = {:?}", state_machine);
            match state_machine {
                StateMachineWrapper::Data(_) => {
                    let data = self.read::<32>()?;
                    for byte in data.iter() {
                        println!("byte = {:x}", byte);
                    }
                    let raw_measurement = RawMeasurement::from(data);
                    let measurement: Measurement = raw_measurement.into();
                    return Ok((raw_measurement, measurement))
                },
                _ => {
                    let byte = self.read::<1>()?;
                    println!("byte = {:x}", byte[0]);
                    state_machine = state_machine.step(byte[0]);
                }
            }
        }
    }
}