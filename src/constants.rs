//! Constants module

/// Number of measurements per scan
pub const NUMBER_OF_MEASUREMENTS_PER_SCAN: usize = 50;
/// Number of points per single measurement / partial scan
pub const NUMBER_OF_POINTS_PER_MEASUREMENT: usize = 8;
/// Number of points per scan
pub const NUMBER_OF_POINTS_PER_SCAN: usize =
    NUMBER_OF_POINTS_PER_MEASUREMENT * NUMBER_OF_MEASUREMENTS_PER_SCAN;
/// Size of payload sent by sensor in bytes
pub const PAYLOAD_SIZE_IN_BYTES: usize = 36;
/// Multiplier for converting point angle to index
pub const INDEX_MULTIPLIER: f32 = NUMBER_OF_POINTS_PER_SCAN as f32 / 360.0;
/// Default offset value from center for measured angles
pub const ANGLE_CENTER_OFFSET_DEFAULT: f32 = 13.0;
/// Default duration between partial scans
pub const UPDATE_INTERVAL_DEFAULT: u64 = 10;
