#![no_std]
#![no_main]

use tp1::rcc::Rcc;
use tp1::gpio::{Gpio, GpioTrait, GpioPort};

use stm32f1::stm32f103;

use cortex_m_rt::entry; // Entry point attribute
use panic_halt as _;    // Panic handler: halt on panic

#[entry]
fn main() -> ! {
    unsafe {
        // 1. Enable the clock for GPIOC
        Rcc::enable_gpio_port_clock(GpioPort::C);

        // 2. Configure PC13 as a push-pull output
        let gpio = Gpio {port: GpioPort::C};
        gpio.set_pin_output(13);

        loop {
            // Turn LED on (PC13 is active low, so set PC13 to 0)
            gpio.set_pin_high(13);

            delay(8_000_000); // Arbitrary delay for ~500ms

            // Turn LED off (set PC13 to 1)
            gpio.set_pin_low(13);

            delay(8_000_000); // Arbitrary delay for ~500ms
        }
    }
}

// Delay function (busy wait)
// TODO Remove
fn delay(count: u32) {
    for _ in 0..count {
        // Simple busy-wait loop
        unsafe { core::arch::asm!("nop") }
    }
}
