use crate::gpio::GpioPort;

use crate::memory_map::stm32f1::AHB_RCC;

const RCC_CR: u32 = 0x00;
const RCC_CFGR: u32 = 0x04;
const RCC_CIR: u32 = 0x08;
const RCC_APB2RSTR: u32 = 0x0C;
const RCC_APB1RSTR: u32 = 0x010;
const RCC_AHBENR: u32 = 0x14;
const RCC_APB2ENR: u32 = 0x18;
const RCC_APB1ENR: u32 = 0x1C;
const RCC_BDCR: u32 = 0x20;
const RCC_CSR: u32 = 0x24;

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