//! This module provides a trait for using USART module on different systems.

#[cfg(target_arch = "avr")]
pub mod atmega328p;

#[cfg(target_arch = "arm")]
pub mod cortex_m3;

pub trait UsartTrait {
    fn initialize(&self);

    fn transmit_byte(&self, byte: u8);

    /// Envoie d'une chaîne de caractère via l'USART 
    fn send_message(&self, s: &str);

    /// Recoit un octet via l'USART
    fn receive_byte(&self) -> u8;
}

#[cfg(target_arch = "avr")]
pub use atmega328p::Usart as Usart;

#[cfg(target_arch = "arm")]
pub use cortex_m3::Usart as Usart;