///! This module provides an abstraction of the device I2C peripheral(s).
/// 
/// # TODO
/// 
/// [ ] Make more methods common in the trait

pub mod atmega328p;
pub mod stm32f1;

pub trait I2cTrait {
    fn init(&self) -> ();
    fn start(&self) -> ();
    fn stop(&self) -> ();
}

#[cfg(target_arch = "avr")]
pub use atmega328p::I2c as I2c;

#[cfg(target_arch = "arm")]
pub use stm32f1::I2c as I2c;
