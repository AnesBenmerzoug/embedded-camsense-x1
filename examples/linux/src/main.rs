extern crate linux;

use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use rerun::{RecordingStreamBuilder, Points2D, Position2D};
use embedded_camsense_x1::CamsenseX1;
use linux_embedded_hal::serialport::{self, DataBits, Parity, StopBits};
use linux_embedded_hal::Delay;

use linux::WrappedUart;

// Configure UART for 115.2 kbit/s,
// no parity bit, 8 data bits and 1 stop bit.
const BAUDRATE: u32 = 115_200;
const PARITY: Parity = Parity::None;
const DATA_BITS: DataBits = DataBits::Eight;
const STOP_BITS: StopBits = StopBits::One;


fn main() -> Result<(), Box<dyn Error>> {
    let mut uart = serialport::new("/dev/ttyUSB0", BAUDRATE).open()?;
    uart.set_parity(PARITY)?;
    uart.set_data_bits(DATA_BITS)?;
    uart.set_stop_bits(STOP_BITS)?;
    uart.set_timeout(Duration::from_millis(1))?;
    println!("Using serial port: {:?}", uart);

    let uart = WrappedUart::new(uart);

    // Initialize Camsense struct
    let mut lidar = CamsenseX1::new(uart, Delay {});

    // Warmup
    for _ in 0..5 {
        let _ = lidar.read_scan();
        sleep(Duration::from_millis(500));
    }

    let rec = RecordingStreamBuilder::new("camsense_x1").spawn()?;
    let mut frame = 0u32;

    loop {
        match lidar.read_scan() {
            Ok(scan) => {
                let positions: Vec<Position2D> = scan.points
                    .iter()
                    .filter_map(|p| *p)
                    .map(|p| {
                        let angle_rad = p.angle.to_radians();
                        let x = p.distance as f32 * angle_rad.cos();
                        let y = p.distance as f32 * angle_rad.sin();
                        Position2D::new(x, y)
                    })
                    .collect();

                rec.set_time_sequence("frame", frame);
                rec.log("lidar/scan", &Points2D::new(positions))?;
                frame += 1;
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}
