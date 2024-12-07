#![no_std]
#![no_main]

use panic_halt as _;
use tp1::usart::{Usart, UsartTrait};

#[no_mangle]
fn main() {
    let usart = Usart {};

    //Initialisation de l'USART
    usart.initialize(); 

    // Envoi d'un message
    usart.send_message("Hello, USART!");
    
    loop {
        
        //Réception d'un octet et réemission de l'octet avec un message pour indiquer la bonne reception
        let received_byte = usart.receive_byte();

        //Envoi d'un message indiquant que l'octet a été reçu
        usart.send_message("Recu : ");
        usart.transmit_byte(received_byte);
    }
}




