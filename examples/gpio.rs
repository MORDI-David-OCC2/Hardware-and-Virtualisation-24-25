#![no_std]
#![no_main]

use panic_halt as _;
use tp1::gpio::{Gpio, GpioTrait};

#[cfg(target_arch = "arm")]
use tp1::rcc::Rcc;

#[cfg(target_arch = "arm")]
use tp1::gpio::stm32f1::GpioPort;

#[cfg(target_arch = "arm")]
use cortex_m_rt::entry;

#[cfg(target_arch = "arm")]
use stm32f1::stm32f103;

#[cfg(target_arch = "arm")]
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

#[cfg(target_arch = "avr")]
#[no_mangle]
fn main() -> ! {
    let gpio = Gpio;
    gpio.set_pin_output(5); //Utilisation de la fonction du module gpio
    gpio.set_pin_high(5); //Allume la broche 5 (exemple)

    loop{}
}
