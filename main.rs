//Projet de David Mordi et Inès Kaci OCC2


#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;

mod gpio; //Import du module gpio ( fichier gpio.rs)



#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    unsafe {

        gpio::set_pin_output(5); //Utilisation de la fonction du module gpio
        gpio::set_pin_high(5); //Allume la broche 5 (exemple)
  
        loop{}

    }
        
    
}



