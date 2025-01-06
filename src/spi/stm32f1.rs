//! Allows the management of the SPI peripherals of any STM32F1 device

use crate::{read_reg, write_reg};

/// Registre de contrôle 1
const SPI_CR1: u32 = 0x4001_3000;

/// Registre d'état
const SPI_SR: u32 = 0x4001_3008;

/// Registre de données
const SPI_DR: u32 = 0x4001_300C;

/// Platform-specific structure abstracting a SPI peripheral.
pub struct Spi;

impl super::SpiTrait for Spi {
    fn init_master() {
        // Configurer SPI en mode maître
        let cr1 = read_reg(SPI_CR1);
        write_reg(SPI_CR1, cr1 | (1 << 2) | (1 << 6)); // BR[2:0] pour diviser l'horloge
    }

    fn init_slave() {
        // Configurer SPI en mode esclave
        let cr1 = read_reg(SPI_CR1);
        write_reg(SPI_CR1, cr1 & !(1 << 2)); // Désactiver le bit maître
    }

    fn transmit(data: u8) {
        // Écrire les données dans le registre DR
        write_reg(SPI_DR, data as u32);
        // Attendre que la transmission soit terminée
        while (read_reg(SPI_SR) & (1 << 1)) == 0 {}
    }

    fn receive() -> u8 {
        // Attendre que les données soient disponibles
        while (read_reg(SPI_SR) & (1 << 0)) == 0 {}
        // Lire les données
        read_reg(SPI_DR) as u8
    }
}