use crate::gpio::GpioPort;

use crate::memory_map::stm32f::{AHB_RCC, RCC_APB2ENR};

pub struct Rcc;

impl Rcc {
    pub unsafe fn enable_gpio_port_clock(port: GpioPort) -> () {
        let bit = Self::get_bit(port);
        let previous_value = core::ptr::read_volatile((AHB_RCC + RCC_APB2ENR) as *mut u32);
        core::ptr::write_volatile((AHB_RCC + RCC_APB2ENR) as *mut u32, previous_value | 1 << bit);
    }

    pub unsafe fn disable_gpio_port_clock(port: GpioPort) -> () {
        let bit = Self::get_bit(port);
        let previous_value = core::ptr::read_volatile((AHB_RCC + RCC_APB2ENR) as *mut u32);
        core::ptr::write_volatile((AHB_RCC + RCC_APB2ENR) as *mut u32, previous_value & !(1 << bit));
    }

    fn get_bit(port: GpioPort) -> u8 {
        match port {
            GpioPort::A => 2,
            GpioPort::B => 3,
            GpioPort::C => 4,
            GpioPort::D => 5,
            GpioPort::E => 6,
            GpioPort::F => 7,
            GpioPort::G => 8,
        }
    }
}