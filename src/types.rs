use crate::constants::INDEX_MULTIPLIER;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    /// Wrapped UART Error
    UART(E),
    /// Min or max speed limits exceeded
    SpeedLimitExceeded(u16, u16, u16),
    /// Checksum mismatch error
    ChecksumMismatch(u32, u32),
    /// Other error
    Other,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RawDistance {
    value: u16,
    quality: u8,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RawMeasurement {
    pub speed: u16,
    pub start_angle: u16,
    pub end_angle: u16,
    pub distances: [RawDistance; 8],
    pub checksum: u16,
}

/// Verifies the Camsense X1 checksum.
/// Accepts the 36-byte payload (header, data, checksum).
///
/// The exact algorithm was taken from the official Camsense X1 C++ SDK:
/// https://github.com/camsense/SDK_V3.0/blob/17e0264302e2ca4cf14d5402af7437d16a37ab95/src/base/ReadParsePackage.cpp#L148
#[inline]
pub fn check_lidar_checksum(data: &[u8; 36]) -> Result<(), Error<()>> {
    let mut accumulator: u32 = 0;

    // Process all words in the slice
    let num_data_words = data.len() / 2 - 1;
    for i in 0..num_data_words {
        let word = u16::from_le_bytes([data[2 * i], data[2 * i + 1]]);
        accumulator = (accumulator << 1) + word as u32;
    }

    // 15-bit folding: equivalent to acc % 32767
    let computed_checksum = ((accumulator & 0x7FFF) + (accumulator >> 15)) & 0x7FFF;

    // Compare with the last word (checksum) in Little-Endian
    let expected_checksum = u16::from_le_bytes([data[data.len() - 2], data[data.len() - 1]]) as u32;

    if computed_checksum == expected_checksum {
        Ok(())
    } else {
        Err(Error::ChecksumMismatch(
            expected_checksum,
            computed_checksum,
        ))
    }
}

impl TryFrom<[u8; 36]> for RawMeasurement {
    type Error = Error<()>; // No UART context during pure byte parsing
    fn try_from(data: [u8; 36]) -> Result<Self, Self::Error> {
        // Validate checksum
        check_lidar_checksum(&data)?;

        let speed = u16::from_le_bytes([data[4], data[5]]);
        let start_angle = u16::from_le_bytes([data[6], data[7]]);
        let end_angle = u16::from_le_bytes([data[32], data[33]]);
        let checksum = u16::from_le_bytes([data[34], data[35]]);

        let mut distances = [RawDistance {
            value: 0,
            quality: 0,
        }; 8];
        for i in 0..8 {
            let distance = RawDistance {
                value: u16::from_le_bytes([data[8 + i * 3], data[9 + i * 3]]),
                quality: data[10 + i * 3],
            };
            distances[i] = distance;
        }

        Ok(Self {
            speed,
            start_angle,
            end_angle,
            distances,
            checksum,
        })
    }
}

#[derive(Clone, Copy, Default, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Point {
    pub distance: u16,
    pub angle: f32,
    pub index: usize,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Measurement {
    pub frequency: f32,
    pub start_angle: f32,
    pub end_angle: f32,
    pub points: [Option<Point>; 8],
}

impl From<(RawMeasurement, f32)> for Measurement {
    fn from((raw, angle_offset): (RawMeasurement, f32)) -> Self {
        let frequency = raw.speed as f32 / 60.0;
        let start_angle = raw.start_angle as f32 / 64.0 - 640.0;
        let end_angle = raw.end_angle as f32 / 64.0 - 640.0;
        let step = if end_angle > start_angle {
            (end_angle - start_angle) / 8.0
        } else {
            (end_angle - (start_angle - 360.0)) / 8.0
        };

        let mut points = [None; 8];
        let mut min_index = None;
        //println!("start: {}, end: {}, step: {}", start_angle, end_angle, step);
        for (i, raw_distance) in raw.distances.iter().enumerate() {
            if raw_distance.quality == 0 {
                continue;
            }
            let angle = (start_angle + step * i as f32 + angle_offset) % 360.0;
            let index = (angle * INDEX_MULTIPLIER).round() as usize % 400;
            if min_index.is_none() {
                min_index = Some(index);
            } else {
                if index < min_index.unwrap() {
                    min_index = Some(index);
                }
            }
            points[i] = Some(Point {
                distance: raw_distance.value,
                angle,
                index,
            });
        }
        Self {
            frequency,
            start_angle,
            end_angle,
            points,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Scan {
    pub points: [Option<Point>; 400],
}
