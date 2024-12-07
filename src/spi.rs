#![allow(dead_code)]

#[cfg(target_arch = "avr")]
pub mod atmega328p;

#[cfg(target_arch = "arm")]
pub mod cortex_m3;

// Définir un trait commun pour les fonctionnalités SPI
pub trait Spi {
    /// Initialiser SPI en tant que maître
    fn init_master();
    /// Initialiser SPI en tant qu'esclave
    fn init_slave();
    /// Transmettre un octet de données (maître)
    fn transmit(data: u8);
    /// Recevoir un octet de données (esclave)
    fn receive() -> u8;
}

// Interface commune pour l'utilisation du SPI
#[cfg(target_arch = "avr")]
pub use atmega328p::Spi as SpiImpl;

#[cfg(target_arch = "arm")]
pub use cortex_m3::Spi as SpiImpl;
