use crate::atmega328p::usart;
use crate::usart::Usart;

#[cfg(target_arch = "avr")]
pub fn get_usart() -> impl Usart {
    usart::Atmega328P {}
}
