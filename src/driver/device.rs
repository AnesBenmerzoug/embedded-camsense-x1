use core::time::Duration;

#[allow(unused)]
// This is needed for calling round() on f32 types
use micromath::F32Ext;

use crate::constants::{INDEX_MULTIPLIER, NUMBER_OF_MEASUREMENTS, NUMBER_OF_POINTS_PER_SCAN};
use crate::state_machine::StateMachineWrapper;
use crate::types::{Error, RawMeasurement};
use crate::{PartialScan, Scan};

use super::{bisync, only_async, only_sync};

#[only_sync]
use embedded_hal::delay::DelayNs;
#[only_sync]
use embedded_io::Read;

#[only_async]
use embedded_hal_async::delay::DelayNs;
#[only_async]
use embedded_io_async::Read;

/// Camsense-X1 controller configuration
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    angle_offset: f32,
    update_interval: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            angle_offset: 13.0,
            update_interval: Duration::from_micros(10),
        }
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

    #[bisync]
    async fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N], Error<UART::Error>> {
        let mut buffer = [0; N];
        self.uart.read(&mut buffer).await.map_err(Error::UART)?;
        Ok(buffer)
    }

    /// Perform a partial scan with the sensor.
    ///
    /// # Returns
    /// - `Ok(PartialScan)`: Structure 8 measured points, each with distance (mm) and angle (°).
    /// - `Err(Error<UART::Error>)`: If there was an error during measurement.
    #[bisync]
    pub async fn read_partial_scan(&mut self) -> Result<PartialScan, Error<UART::Error>> {
        loop {
            let byte = self.read_bytes::<1>().await?;
            self.state_machine = self.state_machine.step(byte[0]);

            if let Some(buf) = self.state_machine.take_buffer() {
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

    /// Perform a complete scan with the sensor.
    ///
    /// # Returns
    /// - `Ok(Scan)`: Structure containing 400 measured points, each with distance (mm) and angle (°).
    /// - `Err(Error<UART::Error>)`: If there was an error during measurement.
    #[bisync]
    pub async fn read_scan(&mut self) -> Result<Scan, Error<UART::Error>> {
        let mut points = [None; NUMBER_OF_POINTS_PER_SCAN];
        for _ in 0..NUMBER_OF_MEASUREMENTS {
            let measurement = self.read_partial_scan().await?;
            for point in measurement.points {
                if let Some(point) = point {
                    let index = (point.angle * INDEX_MULTIPLIER).round() as usize
                        % NUMBER_OF_POINTS_PER_SCAN;
                    points[index] = Some(point);
                }
            }
            self.delay
                .delay_us(self.config.update_interval.as_micros() as u32)
                .await;
        }
        let point_cloud = Scan { points };
        Ok(point_cloud)
    }
}
