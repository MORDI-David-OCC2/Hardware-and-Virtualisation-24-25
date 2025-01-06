//! This module provides an abstraction of the device I2C peripheral(s).
//! 
//! # TODO
//! 
//! [ ] Make more methods common in the trait

pub mod atmega328p;
pub mod stm32f1;

/// Represents one particular I2C peripheral
/// 
/// Instances of this trait represent each one particular I2C peripheral of the
/// device, and they allow controlling it.
/// 
/// # Todo
/// 
/// [ ] Define more methods in the trait
pub trait I2cTrait {
    /// Initializes the I2C peripheral so that it is ready for use
    fn init(&self) -> ();

    /// Create a START condition
    fn start(&self) -> ();

    /// Create a STOP condition
    fn stop(&self) -> ();
}

#[cfg(target_arch = "avr")]
pub use atmega328p::I2c as I2c;

#[cfg(target_arch = "arm")]
pub use stm32f1::I2c as I2c;
