use core::time::Duration;
use crate::constants::{ANGLE_CENTER_OFFSET_DEFAULT, UPDATE_INTERVAL_DEFAULT};

/// Camsense-X1 LiDAR sensor driver configuration.
///
/// Use [`Config::default`] for typical sensor orientations, or construct
/// manually to correct for non-standard mounting.
///
/// # Example
/// ```rust
/// use std::time::Duration;
/// let config = Config { angle_offset: 28.5, update_interval: Duration::from_micros(10) };
/// let lidar = Camsense::with_config(uart, delay, config);
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// Angular correction applied to every point, in degrees.
    ///
    /// Added to each computed angle before storing the point, compensating
    /// for the sensor's physical mounting orientation. Defaults to [`ANGLE_CENTER_OFFSET_DEFAULT`].
    pub angle_offset: f32,
    /// Time duration between partial scans
    ///
    /// Defaults to [`UPDATE_INTERVAL_DEFAULT`].
    pub update_interval: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            angle_offset: ANGLE_CENTER_OFFSET_DEFAULT,
            update_interval: Duration::from_micros(UPDATE_INTERVAL_DEFAULT),
        }
    }
}
