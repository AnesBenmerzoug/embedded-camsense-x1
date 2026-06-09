use embedded_io::Error as UARTErrorTrait;

const ANGLE_OFFSET: f32 = 16.0;
const INDEX_MULTIPLIER: f32 = 400.0 / 360.0;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<UARTError: UARTErrorTrait> {
    /// Wrapped UART Error
    UART(UARTError),
    /// Other error
    Other,
}


#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RawDistance {
    value: u16,
    quality: u8
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RawMeasurement {
    pub speed: u16,
    pub start_angle: u16,
    pub end_angle: u16,
    pub distances: [RawDistance; 8],
    pub check_sum: u16
}

impl From<[u8; 32]> for RawMeasurement {
    fn from(data: [u8; 32]) -> Self {
        let speed = u16::from_le_bytes([data[0], data[1]]);
        let start_angle = u16::from_le_bytes([data[2], data[3]]);
        let end_angle = u16::from_le_bytes([data[28], data[29]]);
        let check_sum = u16::from_le_bytes([data[30], data[31]]);
        let mut distances = [RawDistance { value: 0, quality: 0 }; 8];
        for i in 0..8 {
            let distance = RawDistance {
                value: u16::from_le_bytes([data[4+i*3], data[5+i*3]]),
                quality: data[6+i*3]
            };
            distances[i] = distance;
        }

        Self {
            speed,
            start_angle,
            end_angle,
            distances,
            check_sum
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Point {
    pub distance: u16,
    pub angle: f32,
    pub index: usize
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Measurement {
    pub frequency: f32,
    pub start_angle: f32,
    pub end_angle: f32,
    pub points: [Option<Point>; 8],
}

impl From<RawMeasurement> for Measurement {
    fn from(raw: RawMeasurement) -> Self {
        let frequency = raw.speed as f32 / 3840.0;
        let start_angle = raw.start_angle as f32 / 64.0 - 640.0;
        let end_angle = raw.end_angle as f32 / 64.0 - 640.0;
        let step = if end_angle > start_angle { 
            (end_angle - start_angle) / 8.0
        } else {
            (end_angle - (start_angle - 360.0)) / 8.0
        };

        let mut points = [None; 8];
        //println!("start: {}, end: {}, step: {}", start_angle, end_angle, step);
        for (i, raw_distance) in raw.distances.iter().enumerate() {
            if raw_distance.quality == 0 {
                continue;
            }
            let angle = (start_angle + step * i as f32 + ANGLE_OFFSET) % 360.0;
            let index = (angle * INDEX_MULTIPLIER).round() as usize % 400;
            points[i] = Some(Point { distance: raw_distance.value, angle, index });
        }
        Self {
            frequency,
            start_angle,
            end_angle,
            points
        }
    }
}


#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PointCloud {
    pub points: [Option<Point>; 400]
}