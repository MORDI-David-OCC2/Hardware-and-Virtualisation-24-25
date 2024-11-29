#![allow(dead_code)]


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

// Implémentation spécifique à l'ATmega328p
#[cfg(target_arch = "avr")]
mod atmega328p {
    use core::ptr;
    use super::Spi;

    const DDR_SPI: *mut u8 = 0x24 as *mut u8; // Adresse du registre DDRB
    const SPCR: *mut u8 = 0x4C as *mut u8; // Adresse du registre SPCR
    const SPSR: *mut u8 = 0x4D as *mut u8; // Adresse du registre SPSR
    const SPDR: *mut u8 = 0x4E as *mut u8; // Adresse du registre SPDR

    const DD_MOSI: u8 = 3; // MOSI sur PB3
    const DD_MISO: u8 = 4; // MISO sur PB4
    const DD_SCK: u8 = 5;  // SCK sur PB5

    pub struct SpiAtmega328p;

    impl Spi for SpiAtmega328p {
        fn init_master() {
            unsafe {
                // Configurer MOSI et SCK comme sorties
                let ddr = ptr::read_volatile(DDR_SPI);
                ptr::write_volatile(DDR_SPI, ddr | (1 << DD_MOSI) | (1 << DD_SCK));
                // Activer SPI en mode maître avec fck/16
                ptr::write_volatile(SPCR, (1 << 6) | (1 << 4) | (1 << 0));
            }
        }

        fn init_slave() {
            unsafe {
                // Configurer MISO comme sortie
                let ddr = ptr::read_volatile(DDR_SPI);
                ptr::write_volatile(DDR_SPI, ddr | (1 << DD_MISO));
                // Activer SPI
                ptr::write_volatile(SPCR, 1 << 6);
            }
        }

        fn transmit(data: u8) {
            unsafe {
                // Écrire les données dans SPDR
                ptr::write_volatile(SPDR, data);
                // Attendre que la transmission soit terminée
                while (ptr::read_volatile(SPSR) & (1 << 7)) == 0 {}
            }
        }

        fn receive() -> u8 {
            unsafe {
                // Attendre que la réception soit terminée
                while (ptr::read_volatile(SPSR) & (1 << 7)) == 0 {}
                // Lire les données reçues
                ptr::read_volatile(SPDR)
            }
        }
    }
}

// Implémentation spécifique au Cortex-M7
#[cfg(target_arch = "arm")]
mod cortexm7 {
    use core::ptr;
    use super::Spi;

    const SPI_CR1: *mut u32 = 0x40013000 as *mut u32; // Registre de contrôle 1
    const SPI_SR: *mut u32 = 0x40013008 as *mut u32;  // Registre d'état
    const SPI_DR: *mut u32 = 0x4001300C as *mut u32;  // Registre de données

    pub struct SpiCortexM7;

    impl Spi for SpiCortexM7 {
        fn init_master() {
            unsafe {
                // Configurer SPI en mode maître
                let cr1 = ptr::read_volatile(SPI_CR1);
                ptr::write_volatile(SPI_CR1, cr1 | (1 << 2) | (1 << 6)); // BR[2:0] pour diviser l'horloge
            }
        }

        fn init_slave() {
            unsafe {
                // Configurer SPI en mode esclave
                let cr1 = ptr::read_volatile(SPI_CR1);
                ptr::write_volatile(SPI_CR1, cr1 & !(1 << 2)); // Désactiver le bit maître
            }
        }

        fn transmit(data: u8) {
            unsafe {
                // Écrire les données dans le registre DR
                ptr::write_volatile(SPI_DR, data as u32);
                // Attendre que la transmission soit terminée
                while (ptr::read_volatile(SPI_SR) & (1 << 1)) == 0 {}
            }
        }

        fn receive() -> u8 {
            unsafe {
                // Attendre que les données soient disponibles
                while (ptr::read_volatile(SPI_SR) & (1 << 0)) == 0 {}
                // Lire les données
                ptr::read_volatile(SPI_DR) as u8
            }
        }
    }
}

// Interface commune pour l'utilisation du SPI
#[cfg(target_arch = "avr")]
pub use atmega328p::SpiAtmega328p as SpiImpl;

#[cfg(target_arch = "arm")]
pub use cortexm7::SpiCortexM7 as SpiImpl;
