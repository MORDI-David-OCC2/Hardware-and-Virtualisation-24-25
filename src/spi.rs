pub mod atmega328p;
pub mod stm32f1;

// Définir un trait commun pour les fonctionnalités SPI
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

// Interface commune pour l'utilisation du SPI
#[cfg(target_arch = "avr")]
pub use atmega328p::Spi as SpiImpl;

#[cfg(target_arch = "arm")]
pub use stm32f1::Spi as SpiImpl;
