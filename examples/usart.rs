#![no_std]
#![no_main]

/*
/*use core::panic::PanicInfo;*/
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use panic_halt as _;
mod gpio; //Import du module gpio ( fichier gpio.rs)
*/

//Ajout test 7/11/24


extern crate panic_halt; // Gestionnaire de panique (peut être différent selon votre projet)
extern crate tp1;

use tp1::atmega328p::usart;


#[no_mangle]
fn main() {
    usart::init_uart_reg(); // Initialisation de l'USART

    // Envoi d'un message
    usart::send_message("Hello, USART!");
    
    loop {
        // Boucle infinie pour garder le programme en cours d'exécution
        //partie ajouté le 08/11/24 pour receptionner l'octet et le réemettre 
        let received_byte = usart::receive_byte();
        // Envoi d'un message indiquant que l'octet a été reçu
        usart::send_message("Recu : ");
        usart::transmit_byte(received_byte);
    }
}


//fin ajout test 7/11/24



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




