#![no_std]
#![no_main]

extern crate panic_halt; // Gestionnaire de panique

extern crate tp1;

use tp1::spi::{SpiTrait, SpiImpl};

#[cfg(target_arch = "arm")]
use cortex_m_rt::entry; // Point d'entrée pour Cortex-M7

#[cfg(target_arch = "arm")]
use cortex_m_semihosting::hprintln; // Debug via semihosting pour Cortex-M7


// Point d'entrée pour les deux cibles
#[cfg(target_arch = "avr")]
#[no_mangle]
fn main() -> () {
    // Initialisation SPI pour ATmega328p
    SpiImpl::init_master();
    loop {
        SpiImpl::transmit(0xAB);
    }
}

#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    // Initialisation SPI pour Cortex-M7
    SpiImpl::init_master();
    SpiImpl::transmit(0xAB);

    
    //hprintln!("Octet transmis via SPI : 0xAB").unwrap();

    // Boucle infinie
    loop {}
}