#![no_std]
#![no_main]

use panic_halt as _;
use stm32f1::stm32f103;
use cortex_m_rt::entry; // Entry point attribute
use tp1::usart::{Usart, UsartTrait, UsartPeripheral};

// Entry point
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