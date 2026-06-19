#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]

mod config;
pub mod constants;
pub mod driver;
mod state_machine;
mod types;

pub use config::Config;
pub use driver::blocking::CamsenseX1;
pub use types::{Error, PartialScan, Scan};

#[cfg(feature = "async")]
pub use driver::asynchronous::CamsenseX1 as CamsenseX1Async;
