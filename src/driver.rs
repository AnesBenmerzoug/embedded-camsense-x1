#[path = "device"]
#[cfg(feature = "async")]
pub mod asynchronous {
    //! Asynchronous interface
    use bisync::asynchronous::*;
    #[allow(clippy::duplicate_mod)]
    mod driver;
    mod driver;
    pub use driver::*;
}

#[path = "device"]
pub mod blocking {
    //! Blocking interface
    use bisync::synchronous::*;
    #[allow(clippy::duplicate_mod)]
    mod driver;
    pub use driver::*;
}
