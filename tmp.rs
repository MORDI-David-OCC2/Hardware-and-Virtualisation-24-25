#![no_std]
#![no_main]

use cortex_m_rt::entry; // Provides the `entry` macro
use panic_halt as _; // Halts execution on panic
use stm32f1xx_hal::{
    prelude::*,
    pac,
    gpio::{gpioc::PC13, Output, PushPull},
    delay::Delay,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clock
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Configure GPIO pin PC13 as an output
    let mut gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // Create a delay abstraction based on the system clock
    let mut delay = Delay::new(cortex_m::Peripherals::take().unwrap().SYST, clocks);

    loop {
        led.set_high(); // Turn LED on
        delay.delay_ms(500_u16);

        led.set_low(); // Turn LED off
        delay.delay_ms(500_u16);
    }
}
