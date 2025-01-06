//! This module provides an abstraction for interacting with a device’s
//! USART/UART peripheral(s).

pub mod atmega328p;
pub mod stm32f1;

pub trait UsartTrait {
    fn init(&self);

    fn transmit_byte(&self, byte: u8);

    /// Envoie d'une chaîne de caractère via l'USART 
    fn send_message(&self, s: &str);

    /// Recoit un octet via l'USART
    fn receive_byte(&self) -> u8;
}

#[cfg(target_arch = "avr")]
pub use atmega328p::Usart as Usart;

#[cfg(target_arch = "arm")]
pub use stm32f1::Usart as Usart;

#[cfg(target_arch = "arm")]
pub use stm32f1::UsartPeripheral as UsartPeripheral;