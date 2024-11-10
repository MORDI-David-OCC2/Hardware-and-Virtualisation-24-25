#![no_std]
#![no_main]

extern crate panic_halt; // Gestionnaire de panique
extern crate tp1;

use tp1::auto::usart;
use tp1::usart::Usart;

//--------------------------------Test des fonctions pour le rendu 2------------
#[no_mangle]
fn main() {
    let hal = usart::get_usart();

    //Initialisation de l'USART
    hal.initialize(); 

    // Envoi d'un message
    hal.send_message("Hello, USART!");
    
    loop {
        
        //Réception d'un octet et réemission de l'octet avec un message pour indiquer la bonne reception

        let received_byte = hal.receive_byte();
        //Envoi d'un message indiquant que l'octet a été reçu
        hal.send_message("Recu : ");
        hal.transmit_byte(received_byte);
    }
}


//--------------------------------Test GPIO-------------------------------------
// #[entry]
// fn main() -> ! {
//     unsafe {
//         let _ = hprintln!("Hello, world!").unwrap();
//         //gpio::set_pin_output(5); //Utilisation de la fonction du module gpio
//         //gpio::set_pin_high(5); //Allume la broche 5 (exemple)
//         debug::exit(debug::EXIT_SUCCESS);
//         loop{}
//     }
        
    
// }*/





