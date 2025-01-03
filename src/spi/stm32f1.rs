use core::ptr;

const SPI_CR1: *mut u32 = 0x40013000 as *mut u32; // Registre de contrôle 1
const SPI_SR: *mut u32 = 0x40013008 as *mut u32;  // Registre d'état
const SPI_DR: *mut u32 = 0x4001300C as *mut u32;  // Registre de données

pub struct Spi;

impl super::Spi for Spi {
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