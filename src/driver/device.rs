#[allow(unused)]
// This is needed for calling round() on f32 types
use micromath::F32Ext;

use crate::constants::{
    INDEX_MULTIPLIER, NUMBER_OF_MEASUREMENTS_PER_SCAN,
    NUMBER_OF_POINTS_PER_SCAN,
};
use crate::state_machine::StateMachineWrapper;
use crate::types::{Error, RawMeasurement};
use crate::{PartialScan, Scan};
use crate::config::Config;

use super::{bisync, only_async, only_sync};

#[only_sync]
use embedded_hal::delay::DelayNs;
#[only_sync]
use embedded_io::Read;

#[only_async]
use embedded_hal_async::delay::DelayNs;
#[only_async]
use embedded_io_async::Read;
/// Camsense-X1 LiDAR sensor driver.
///
/// Handles byte-level framing, checksum validation, and angle computation.
/// Produces either individual [`PartialScan`]s or full 360° [`Scan`]s.
///
/// # Example
/// ```rust
/// let mut lidar = Camsense::new(uart, delay);
/// let scan = lidar.read_scan()?;
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CamsenseX1<UART: Read, D: DelayNs> {
    /// Concrete UART implementation.
    uart: UART,
    delay: D,
    state_machine: StateMachineWrapper,
    config: Config,
}

impl<UART, D> CamsenseX1<UART, D>
where
    UART: Read,
    D: DelayNs,
{
    /// Creates a new driver with default [`Config`].
    pub fn new(uart: UART, delay: D) -> Self {
        Self::with_config(uart, delay, Config::default())
    }

    /// Creates a new driver with a custom [`Config`].
    pub fn with_config(uart: UART, delay: D, config: Config) -> Self {
        Self {
            uart,
            delay,
            state_machine: StateMachineWrapper::new(),
            config,
        }
    }

    /// Reads exactly `N` raw bytes from the UART.
    #[bisync]
    async fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N], Error<UART::Error>> {
        let mut buffer = [0; N];
        self.uart.read(&mut buffer).await.map_err(Error::UART)?;
        Ok(buffer)
    }

    /// Reads a [`PartialScan`] from the sensor.
    ///
    /// Feeds bytes one at a time into the packet state machine until
    /// a complete [`PAYLOAD_SIZE_IN_BYTES`]-byte frame is received.
    /// Frames that fail checksum validation are silently discarded and the state machine resyncs
    /// automatically, so this method will keep reading until a valid packet
    /// is found.
    ///
    /// Each frame contains measurements for [`NUMBER_OF_POINTS_PER_MEASUREMENT`] points.
    ///
    /// # Returns
    /// - `Ok(PartialScan)`: Structure containing [`NUMBER_OF_POINTS_PER_MEASUREMENT`] measured points, each with distance (mm) and angle (°).
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

    /// Reads a complete 360° [`Scan`] from the sensor.
    ///
    /// Collects [`NUMBER_OF_MEASUREMENTS_PER_SCAN`] consecutive [`PartialScan`] packets
    /// and merges their points into a [`NUMBER_OF_POINTS_PER_SCAN`]-slot array indexed by angle.
    ///
    /// # Returns
    /// - `Ok(Scan)`: Structure containing [`NUMBER_OF_POINTS_PER_SCAN`] measured points, each with distance (mm) and angle (°).
    /// - `Err(Error<UART::Error>)`: If there was an error during measurement.
    #[bisync]
    pub async fn read_scan(&mut self) -> Result<Scan, Error<UART::Error>> {
        let mut points = [None; NUMBER_OF_POINTS_PER_SCAN];
        for _ in 0..NUMBER_OF_MEASUREMENTS_PER_SCAN {
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
