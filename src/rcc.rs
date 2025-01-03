#[cfg(target_arch = "arm")]
pub mod stm32f1;

#[cfg(target_arch = "arm")]
pub use stm32f1::Rcc;