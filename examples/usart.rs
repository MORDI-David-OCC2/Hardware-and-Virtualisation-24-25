#![no_std]
#![no_main]

use panic_halt as _;
use tp1::usart::{Usart, UsartTrait};

#[cfg(target_arch = "arm")]
use tp1::usart::stm32f1::UsartPeripheral;

#[cfg(target_arch = "arm")]
use stm32f1::stm32f103;

#[cfg(target_arch = "arm")]
use cortex_m_rt::entry; // Entry point attribute

// Entry point
#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    let usart = Usart{
        peripheral: UsartPeripheral::Usart1,
        use_9_bit_words: false,
    };
    usart.init();

    loop {
        let message = "Hello USART\r\n";
        usart.send_message(message);
        usart.set_listening_status(true);
        let byte = usart.receive_byte();
        usart.transmit_byte(byte);
        usart.set_listening_status(false);
    }
}

#[cfg(target_arch = "avr")]
#[no_mangle]
fn main() {
    let usart = Usart {};

    //Initialisation de l'USART
    usart.init(); 

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




