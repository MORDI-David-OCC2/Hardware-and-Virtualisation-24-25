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
    unsafe {
        usart.init();
    }

    loop {
        unsafe {
            let message = "Hello USART\r\n";
            usart.send_message(message);

        }
        delay_ms(1000); // Wait 1 second
    }
}

// Delay function (busy wait)
// TODO Remove
fn delay_ms(count: u32) {
    for _ in 0..count {
        // Simple busy-wait loop
        unsafe { core::arch::asm!("nop") }
    }
}