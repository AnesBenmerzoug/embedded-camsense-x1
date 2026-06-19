#[path = "driver"]
#[cfg(feature = "async")]
pub mod asynchronous {
    //! Asynchronous interface
    use bisync::asynchronous::*;
    #[allow(clippy::duplicate_mod)]
    mod device;
    pub use device::*;
}

#[path = "driver"]
pub mod blocking {
    //! Blocking interface
    use bisync::synchronous::*;
    #[allow(clippy::duplicate_mod)]
    mod device;
    pub use device::*;
}
