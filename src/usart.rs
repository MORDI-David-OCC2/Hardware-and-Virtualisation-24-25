//! Abstracts the device’s USART/UART peripheral(s)

pub mod atmega328p;
pub mod stm32f1;

/// Abstracts one particular USART/UART peripheral
/// 
/// This trait is implemented by each supported platform.
/// Certain methods are not part of this trait because their signature is
/// platform-dependent. They were left out for this reason.
pub trait UsartTrait {
    /// Initializes the peripheral
    fn init(&self);

    /// Send one byte over USART
    fn transmit_byte(&self, byte: u8);

    /// Send a sequence of byte over USART
    fn send_message(&self, s: &str);

    /// Read one byte from USART
    fn receive_byte(&self) -> u8;
}

#[cfg(target_arch = "avr")]
pub use atmega328p::Usart as Usart;

#[cfg(target_arch = "arm")]
pub use stm32f1::Usart as Usart;

#[cfg(target_arch = "arm")]
pub use stm32f1::UsartPeripheral as UsartPeripheral;