//! Cette partie conerne le protocole usart pour la cible atmega328p

use core::ptr::{read_volatile, write_volatile};
use crate::usart::UsartTrait;


// Configuration des constantes

// Paramètres de l’USART
pub const FOSC: u32 = 1_843_200;  // fréquence
pub const BAUD: u32 = 9_600; // taux de baud
pub const UBRR: u32 = (FOSC/(16*BAUD))-1;

// Séparation du registre en deux registres (haut et bas)
pub const UBRR0_H: u8 = (UBRR >> 8u8) as u8;
pub const UBRR0_L: u8 = (UBRR) as u8;

// Configuration des registres pour le recepteur et l'emetteur
pub const URBB0_B: u8 = (1<<RXCEN0) | (1<<TXEN0);
pub const UBRR0_C: u8 = (1<<USBS0) | (3<<UCSZ00);
 
pub const RXCEN0: u8 = 4; //récepteur
pub const TXEN0: u8 = 3; //emetteur 

// Bits de configuration pour le registre UCSR0C
pub const UCSZ00: u8 = 1;
pub const USBS0: u8 = 3;

// Adresses des registres liées à l'USART
pub const ADD_UCSR0C: *mut u8 = 0xC2 as *mut u8;
pub const ADD_UCSR0B: *mut u8 = 0xC1 as *mut u8;
pub const ADD_UBRR0H: *mut u8 = 0xC5 as *mut u8; // usart br reg high
pub const ADD_UBRR0L: *mut u8 = 0xC4 as *mut u8; // usart br reg low
pub const ADD_UCSR0A: *mut u8 = 0xC0 as *mut u8;
pub const UDRE0: u8 = 5;
pub const ADD_UDR0: *mut u8 = 0xC6 as *mut u8;

pub struct Usart {
}

impl UsartTrait for Usart {
    /// Initialisation de l'USART
    fn init(&self)
    {
        // Ecriture des valeurs dans les registres
        unsafe {
            write_volatile(ADD_UBRR0H, UBRR0_H);
            write_volatile(ADD_UBRR0L, UBRR0_L);
            write_volatile(ADD_UCSR0C, UBRR0_C);
            write_volatile(ADD_UCSR0B, URBB0_B);
        }
    }

    /// Transmission d'un octet via USART
    fn transmit_byte(&self, byte: u8) {
        unsafe
        {
            // Attente que le tampon de transmission soit vide
            while read_volatile(ADD_UCSR0A) & (1<<UDRE0)==0{}
            // Ecriture de l'octet dans le registre de transmission
            write_volatile(ADD_UDR0, byte);
        }
    }

    /// Envoie d'une chaîne de caractère via l'USART 
    fn send_message(&self, s: &str) {
        s.as_bytes().iter().for_each(|b| self.transmit_byte(*b));
    }

    /// Recoit un octet via l'USART
    fn receive_byte(&self) -> u8 {
        unsafe {
            // Attente que des données soient disponibles (bit RXC0 - bit 7 de UCSR0A)
            while read_volatile(ADD_UCSR0A) & (1 << 7) == 0 {} 

            // Lire les données du registre de réception
            read_volatile(ADD_UDR0)
        }
    }
}