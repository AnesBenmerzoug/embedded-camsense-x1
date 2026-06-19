extern crate linux;

use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use egui_plot::{Plot, PlotPoints, Points};
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

struct LidarApp {
    lidar: CamsenseX1<WrappedUart, Delay>,
    points: Vec<[f64; 2]>,
    update_interval: Duration,
}

impl LidarApp {
    fn new(lidar: CamsenseX1<WrappedUart, Delay>) -> Self {
        Self {
            lidar,
            points: Vec::new(),
            update_interval: Duration::from_secs(1),
        }
    }
}

impl eframe::App for LidarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Read a new scan
        match self.lidar.read_scan() {
            Ok(point_cloud) => {
                self.points = point_cloud
                    .points
                    .iter()
                    .filter_map(|p| *p)
                    .map(|p| {
                        let angle_rad = p.angle.to_radians() as f64;
                        let x = p.distance as f64 * angle_rad.cos();
                        let y = p.distance as f64 * angle_rad.sin();
                        [x, y]
                    })
                    .collect();
            }
            Err(e) => eprintln!("Read error: {:?}", e),
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("lidar")
                .data_aspect(1.0) // keep x and y axes scaled equally
                .center_x_axis(true)
                .center_y_axis(true)
                .show(ui, |plot_ui| {
                    let plot_points = PlotPoints::new(self.points.clone());
                    plot_ui.points(
                        Points::new("point cloud", plot_points)
                            .radius(2.0)
                            .name("lidar"),
                    );
                });
        });

        ctx.request_repaint_after(self.update_interval);
    }
}

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

    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Camsense-X1",
        options,
        Box::new(|_cc| Ok(Box::new(LidarApp::new(lidar)))),
    );

    Ok(())
}
