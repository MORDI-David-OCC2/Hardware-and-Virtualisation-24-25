#[cfg(target_arch = "arm")]
pub mod cortex_m3;

#[cfg(target_arch = "arm")]
pub use cortex_m3::Rcc;