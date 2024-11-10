//! Cette partie conerne le protocole usart pour la cible atmega328p

use core::ptr::{read_volatile, write_volatile};
use super::usart;


// Configuration des constantes

pub const FOSC : u32 = 1843200;  // fréquence
pub const BAUD : u32 = 9600; // taux de baud
 
pub const RXCEN0 : u8 = 4; //récepteur
pub const TXEN0  : u8 = 3; //emetteur 

// Bits de configuration pour le registre UCSR0C
pub const UCSZ00 : u8 = 1;
pub const USBS0  : u8 = 3;

// Adresses des registres liées à l'USART
pub const ADD_UCSR0C: *mut u8 = 0xC2 as *mut u8;
pub const ADD_UCSR0B: *mut u8 = 0xC1 as *mut u8;
pub const ADD_UBRR0H: *mut u8 = 0xC5 as *mut u8;//usart br reg high
pub const ADD_UBRR0L: *mut u8 = 0xC4 as *mut u8;//usart br reg low
pub const ADD_UCSR0A: *mut u8 = 0xC0 as *mut u8;
pub const UDRE0  : u8 = 5;
pub const ADD_UDR0  : *mut u8 = 0xC6 as *mut u8;

/// Initialisation de l'USART
pub fn init_uart_reg()
{
    // Configuration du registre du taux de baud
    let ubrr=(FOSC/(16*BAUD))-1;

    // Séparation du registre en deux registres (haut et bas)
    let ubrr0h: u8 = (ubrr >> 8) as u8;
    let ubrr0l: u8 = (ubrr) as u8;

 
    // Configuration des registres pour le recepteur et l'emetteur
    let ucsr0b: u8 = (1<<RXCEN0) | (1<<TXEN0);
    let ucsr0c: u8 = (1<<USBS0) | (3<<UCSZ00);

    // Ecriture des valeurs dans les registres
    unsafe {
        write_volatile(ADD_UBRR0H, ubrr0h);
        write_volatile(ADD_UBRR0L, ubrr0l);
        write_volatile(ADD_UCSR0C, ucsr0c);
        write_volatile(ADD_UCSR0B, ucsr0b);
    }
}

// Fonctions de transmission

/// Transmission d'un octet via USART
pub fn transmit_byte(byte : u8)
{
    unsafe
    {
        // Attente que le tampon de transmission soit vide
        while read_volatile(ADD_UCSR0A) & (1<<UDRE0)==0{}
        // Ecriture de l'octet dans le registre de transmission
        write_volatile(ADD_UDR0,byte);
    }
}

/// Envoie d'une chaîne de caractère via l'USART 
pub fn send_message(s:&str){
    s.as_bytes().iter().for_each(|b| transmit_byte(*b));
}

// Fonction de réception

/// Recoit un octet via l'USART
pub fn receive_byte() -> u8 {
    unsafe {
        // Attente que des données soient disponibles (bit RXC0 - bit 7 de UCSR0A)
        while read_volatile(ADD_UCSR0A) & (1 << 7) == 0 {} 

        // Lire les données du registre de réception
        read_volatile(ADD_UDR0)
    }
}