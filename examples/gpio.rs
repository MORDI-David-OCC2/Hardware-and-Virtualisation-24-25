// #![no_std]
// #![no_main]

// extern crate panic_halt; // Gestionnaire de panique

// use tp1::gpio::{Gpio, GpioTrait, GpioPort}; //Import du module gpio ( fichier gpio.rs)

// // #[cfg(target_arch = "arm")]
// use cortex_m_semihosting::{debug, hprintln};
// use stm32f1::stm32f103;

// use cortex_m_rt::entry;

// #[entry]
// fn main() -> ! {
//     let gpio = Gpio::new(GpioPort::C);
//     unsafe {
//         gpio.set_pin_output(0);
//     }
//     hprintln!("Hello");
//     // #[cfg(target_arch = "arm")]


//     // Gpio::set_pin_output(5); //Utilisation de la fonction du module gpio
//     // Gpio::set_pin_high(5); //Allume la broche 5 (exemple)

//     // #[cfg(target_arch = "arm")]
//     // debug::exit(debug::EXIT_SUCCESS);
//     loop{}
// }

#![no_std]
#![no_main]

use tp1::rcc::Rcc;
use tp1::gpio::{Gpio, GpioTrait, GpioPort};

use core::ptr;
use stm32f1::stm32f103;

use cortex_m_rt::entry; // Entry point attribute
use panic_halt as _;    // Panic handler: halt on panic

#[entry]
fn main() -> ! {
    // Base addresses for RCC, GPIOC
    const RCC_BASE: u32 = 0x40021000;
    const GPIOC_BASE: u32 = 0x40011000;

    // Offsets for registers
    const RCC_APB2ENR: u32 = RCC_BASE + 0x18; // APB2 peripheral clock enable register
    const GPIOC_CRH: u32 = GPIOC_BASE + 0x04; // Port configuration register high
    
    const GPIOC_BSRR: u32 = GPIOC_BASE + 0x10; // Bit set/reset register

    unsafe {
        // 1. Enable the clock for GPIOC
        // let rcc_apb2enr = RCC_APB2ENR as *mut u32;
        // ptr::write_volatile(rcc_apb2enr, ptr::read_volatile(rcc_apb2enr) | (1 << 4));
        Rcc::enable_gpio_port_clock(GpioPort::C);

        // 2. Configure PC13 as a push-pull output
        // let gpioc_crh = GPIOC_CRH as *mut u32;
        // let mut crh_val = ptr::read_volatile(gpioc_crh);
        // crh_val &= !(0b1111 << 20); // Clear CNF13[1:0] and MODE13[1:0]
        // crh_val |= (0b0010 << 20);  // Set MODE13 to Output mode, max speed 2 MHz
        // ptr::write_volatile(gpioc_crh, crh_val);
        let gpio = Gpio {port: GpioPort::C};
        gpio.set_pin_output(13);

        // 3. Blink the LED by toggling PC13
        let gpioc_bsrr = GPIOC_BSRR as *mut u32;
        loop {
            // Turn LED on (PC13 is active low, so set PC13 to 0)
            ptr::write_volatile(gpioc_bsrr, 1 << 13);

            delay(8_000_000); // Arbitrary delay for ~500ms

            // Turn LED off (set PC13 to 1)
            ptr::write_volatile(gpioc_bsrr, 1 << (13 + 16)); // Reset bit for PC13

            delay(8_000_000); // Arbitrary delay for ~500ms
        }
    }
}

/// Delay function (busy wait)
fn delay(count: u32) {
    for _ in 0..count {
        // Simple busy-wait loop
        unsafe { core::arch::asm!("nop") }
    }
}
