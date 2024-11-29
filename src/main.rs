

#![no_std]
#![no_main]

//------------------------------------------Partie utile pour le rendu 1-----------------------------------------------
/*
/*use core::panic::PanicInfo;*/
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use panic_halt as _;
mod gpio; //Import du module gpio ( fichier gpio.rs)
*/
/*#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}*/
/*
#[entry]
fn main() -> ! {
    unsafe {
        let _ = hprintln!("Hello, world!").unwrap();
        //gpio::set_pin_output(5); //Utilisation de la fonction du module gpio
        //gpio::set_pin_high(5); //Allume la broche 5 (exemple)
        debug::exit(debug::EXIT_SUCCESS);
        loop{}
    }
        
    
}*/

//--------------------------------Test des fonctions pour le rendu 2------------------------------------------
//extern crate panic_halt; // Gestionnaire de panique 
mod usart;
/*

#[no_mangle]
fn main() {
    //Initialisation de l'USART
    usart::init_uart_reg(); 

    // Envoi d'un message
    usart::send_message("Hello, USART!");
    
    loop {
        
        //Réception d'un octet et réemission de l'octet avec un message pour indiquer la bonne reception

        let received_byte = usart::receive_byte();
        //Envoi d'un message indiquant que l'octet a été reçu
        usart::send_message("Recu : ");
        usart::transmit_byte(received_byte);
    }
}
*/
//Test pour rendu 3 

// Importations communes
mod spi;
use spi::{Spi, SpiImpl};

#[cfg(target_arch = "avr")]
use panic_halt as _; // Gestionnaire de panique pour ATmega328p

#[cfg(target_arch = "arm")]
use panic_halt as _; // Gestionnaire de panique pour Cortex-M7

#[cfg(target_arch = "arm")]
use cortex_m_rt::entry; // Point d'entrée pour Cortex-M7
#[cfg(target_arch = "arm")]
use cortex_m_semihosting::hprintln; // Debug via semihosting pour Cortex-M7

// Point d'entrée pour les deux cibles
#[cfg(target_arch = "avr")]
#[no_mangle]
pub extern "C" fn main() -> ! {
    // Initialisation SPI pour ATmega328p
    SpiImpl::init_master();
    SpiImpl::transmit(0xAB);

    // Boucle infinie
    loop {}
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








