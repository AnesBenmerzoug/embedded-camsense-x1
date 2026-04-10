
extern crate raspberrypi;

use std::env;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use rppal::hal::Delay;
use rppal::uart::{Uart, Parity};
use embedded_camsense_x1::Camsense;

// Configure UART for 115.2 kbit/s,
// no parity bit, 8 data bits and 1 stop bit.
const BAUDRATE: u32 = 115_200;
const PARITY: Parity = Parity::None;
const DATA_BITS: u8 = 8;
const STOP_BITS: u8 = 1;

use raspberrypi::WrappedUart;


fn main() -> Result<(), Box<dyn Error>> {
    let mut uart = Uart::new(
        BAUDRATE, PARITY, DATA_BITS, STOP_BITS
    )?;

    // Configure read() to block until at least 1 byte is received.
    uart.set_read_mode(1, Duration::default())?;

    let uart = WrappedUart::new(uart);

    let mut lidar = Camsense::new(uart, Delay::new());

    for _ in 0..10 {
        // Fill the buffer variable with any incoming data.
        //if uart.read(&mut buffer)? > 0 {
        //    println!("Received byte: {:x}", buffer[0]);
        //}
        let (raw_measurement, measurement) = lidar.get_point_cloud().unwrap();
        println!("raw_measurement = {:?}", raw_measurement);
        println!("");
        println!("measurement = {:?}", measurement);
        println!("");
        sleep(Duration::from_millis(500));
    }
    Ok(())
}
