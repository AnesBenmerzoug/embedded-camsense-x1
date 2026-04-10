use embedded_io::Error as UARTErrorTrait;

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


fn combine_bytes_into_word(low_byte: u8, high_byte: u8) -> u16 {
    ((high_byte as u16) << 8) + low_byte as u16
}

impl From<[u8; 32]> for RawMeasurement {
    fn from(data: [u8; 32]) -> Self {
        let speed = combine_bytes_into_word(data[0], data[1]);
        let start_angle = combine_bytes_into_word(data[2], data[3]);
        let end_angle = combine_bytes_into_word(data[28], data[29]);
        let check_sum = combine_bytes_into_word(data[30], data[31]);
        let mut distances = [RawDistance { value: 0, quality: 0 }; 8];
        for i in 0..8 {
            let distance = RawDistance {
                value: combine_bytes_into_word(data[4+i*3], data[5+i*3]),
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

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Distance {
    value: u16,
    angle: f32
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Measurement {
    pub frequency: f32,
    pub start_angle: f32,
    pub end_angle: f32,
    pub distances: [Distance; 8],
}

impl From<RawMeasurement> for Measurement {
    fn from(raw: RawMeasurement) -> Self {
        let frequency = raw.speed as f32 / 3840.0;
        let start_angle = raw.start_angle as f32 / 64.0 - 640.0;
        let end_angle = raw.end_angle as f32 / 64.0 - 640.0;
        let step = if end_angle > start_angle { 
            (end_angle as f32 - start_angle as f32) / 8.0
        } else {
            (end_angle as f32 - (start_angle as f32 - 360.0)) / 8.0
        };

        let mut distances = [Distance { value: 0, angle: 0.0 }; 8];
        for (i, raw_distance) in raw.distances.iter().enumerate() {
            let angle = start_angle + step * i as f32;
            let distance = Distance { value:raw_distance.value, angle };
            distances[i] = distance;
        }
        Self {
            frequency,
            start_angle,
            end_angle,
            distances
        }
    }
}