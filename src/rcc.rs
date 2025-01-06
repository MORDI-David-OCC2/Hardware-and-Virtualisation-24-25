//! Provides an abstraction for interacting with a device RCC peripheral.
//! 
//! This is an incomplete, work in progress module. This is because no emulator
//! provides a way to verify a RCC implementation, they simply ignore this
//! hardware component, which remains crucial nonetheless.

pub mod stm32f1;

#[cfg(target_arch = "arm")]
pub use stm32f1::Rcc;