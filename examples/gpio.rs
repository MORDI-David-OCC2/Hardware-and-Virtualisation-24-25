#![no_std]
#![no_main]

extern crate panic_halt; // Gestionnaire de panique

use tp1::gpio::{Gpio, GpioTrait}; //Import du module gpio ( fichier gpio.rs)

#[cfg(target_arch = "arm")]
use cortex_m_rt::entry;

#[cfg(target_arch = "arm")]
use cortex_m_semihosting::{debug, hprintln};

#[no_mangle]
fn main() -> ! {
    unsafe {
        #[cfg(target_arch = "arm")]
        let _ = hprintln!("Hello, world!");

        Gpio::set_pin_output(5); //Utilisation de la fonction du module gpio
        Gpio::set_pin_high(5); //Allume la broche 5 (exemple)

        #[cfg(target_arch = "arm")]
        debug::exit(debug::EXIT_SUCCESS);
        loop{}
    }
}