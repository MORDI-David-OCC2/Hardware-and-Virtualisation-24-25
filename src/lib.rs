#![no_std]

pub mod atmega328p;

// use core::ptr::{read_volatile, write_volatile};

// use hal::Hal;

// // Constants

// pub const FOSC: u32 = 1843200;  // Clock Speed
// pub const BAUD: u32 = 9600; // Baud Rate

// pub const UCSZ00: u8 = 1;
// pub const RXCEN0: u8 = 4;

// pub const TXEN0: u8 = 3;
// pub const USBS0: u8 = 3;

// pub const ADD_UCSR0C: *mut u8 = 0xC2 as *mut u8;
// pub const ADD_UCSR0B: *mut u8 = 0xC1 as *mut u8;
// pub const ADD_UBRR0H: *mut u8 = 0xC5 as *mut u8; //usart br reg high
// pub const ADD_UBRR0L: *mut u8 = 0xC4 as *mut u8; //usart br reg low

// pub const ADD_UCSR0A: *mut u8 = 0xC0 as *mut u8;
// pub const UDRE0: u8 = 5;
// pub const ADD_UDR0: *mut u8 = 0xC6 as *mut u8;

// pub struct Atmega328pHal {
// }

// impl hal::Hal for Atmega328pHal {
//     fn init_uart_reg() {
//         let ubrr=(FOSC/(16*BAUD))-1;

//         // Setup USART Baud Rate registers value
    
//         // Baud prescaler register is split on two bytes
//         let ubrr0h: u8 = (ubrr >> 8) as u8;
//         let ubrr0l: u8 = (ubrr) as u8;
    
     
    
//         // Enable receiver and transmitter
//         let ucsr0b: u8 = (1<<RXCEN0) | (1<<TXEN0);
    
//         // Set frame format: 8data, 1stop bit
//         let ucsr0c: u8 = (1<<USBS0) | (3<<UCSZ00);

//         unsafe {
//             write_volatile(ADD_UBRR0H, ubrr0h);
//             write_volatile(ADD_UBRR0L, ubrr0l);
//             write_volatile(ADD_UCSR0C, ucsr0c);
//             write_volatile(ADD_UCSR0B, ucsr0b);
//         }
//     }

//     fn transmit_byte(byte : u8)
//     {
//         unsafe
//         {
//             // let ucsr0a = ptr::read_volatile(ADD_UCSR0A);
//             // let udre0 = ucsr0a & (1 << UDRE0) ;

//             // Wait for empty transmit buffer
//             while read_volatile(ADD_UCSR0A) & (1<<UDRE0)==0{}

//             write_volatile(ADD_UDR0,byte);
//         }
//     }

//     fn send_message(s:&str) {
//         s.as_bytes()
//             .iter()
//             .for_each(|b| Self::transmit_byte(*b));
//     }

//     //fonction ajouté le 08/11/24 pour tester la reception de message 
//     fn receive_byte() -> u8 {
//         unsafe {
//             // Attendre que des données soient disponibles dans le tampon de réception
//             while read_volatile(ADD_UCSR0A) & (1 << 7) == 0 {} // Le bit 7 (RXC0) de UCSR0A indique la disponibilité des données

//             // Lire les données du registre de réception
//             read_volatile(ADD_UDR0)
//         }
//     }
// }
