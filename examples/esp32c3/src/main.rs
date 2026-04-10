#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embedded_camsense_x1::camsense_x1::Camsense;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_hal::uart::{Uart, Config};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.5.0

    rtt_target::rtt_init_defmt!();

    info!("Config");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    info!("UART");
    // Uart Rx Pin
    let rx_pin = peripherals.GPIO21;
    // Uart Tx Pin
    let tx_pin = peripherals.GPIO9;
    let mut uart = Uart::new(
            peripherals.UART0,
            Config::default())
        .unwrap()
        .with_rx(rx_pin)
        .with_tx(tx_pin);
    info!("Ready ready = {}", uart.read_ready());
    info!("Camsense");
    //let mut camsense_x1 = Camsense::new(uart, Delay::new());
    //let mut buffer = [0; 36];
    //camsense_x1.read(&mut buffer).unwrap();
    //info!("Buffer = {:x}", buffer);
    info!("Loop");

    loop {
        info!("Ready ready = {}", uart.read_ready());
        if uart.read_ready() {
            let mut buffer = [0; 4];
            uart.read(&mut buffer).unwrap();
            info!("Buffer = {:x}", buffer);
        }
        //let mut buffer = [0; 36];
        //camsense_x1.read(&mut buffer).unwrap();
        //info!("Buffer = {:x}", buffer);
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(100) {}
        info!("after delay");
    }
}
