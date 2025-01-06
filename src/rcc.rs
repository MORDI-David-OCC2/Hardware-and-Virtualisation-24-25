//! Provides an abstraction for interacting with a device RCC peripheral.
//! 
//! This is an incomplete, work in progress module.

pub mod stm32f1;

#[cfg(target_arch = "arm")]
pub use stm32f1::Rcc;