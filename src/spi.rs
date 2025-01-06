//! Provides a way to manage the SPI peripheral(s).

pub mod atmega328p;
pub mod stm32f1;

/// Represents a particular SPI peripheral and allows managing it
pub trait SpiTrait {
    /// Initialiser SPI en tant que maître
    fn init_master();

    /// Initialiser SPI en tant qu'esclave
    fn init_slave();

    /// Transmettre un octet de données (maître)
    fn transmit(data: u8);

    /// Recevoir un octet de données (esclave)
    fn receive() -> u8;
}

#[cfg(target_arch = "avr")]
pub use atmega328p::Spi as SpiImpl;

#[cfg(target_arch = "arm")]
pub use stm32f1::Spi as SpiImpl;
