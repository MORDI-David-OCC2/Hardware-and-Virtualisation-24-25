//! This module provides a trait for using USART module on different systems.

pub trait Usart {
    fn initialize(&self);

    fn transmit_byte(&self, byte: u8);

    /// Envoie d'une chaîne de caractère via l'USART 
    fn send_message(&self, s: &str);

    /// Recoit un octet via l'USART
    fn receive_byte(&self) -> u8;
}