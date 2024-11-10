use crate::atmega328p;
use crate::cortex_m3;
use crate::usart::Usart;

#[cfg(target_arch = "avr")]
pub fn get_usart() -> impl Usart {
    atmega328p::usart::Atmega328P {}
}

#[cfg(target_arch = "arm")]
pub fn get_usart() -> impl Usart {
    cortex_m3::usart::CortexM3 {}
}