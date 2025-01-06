//! Allows managing the RCC peripheral

use crate::gpio::stm32f1::GpioPort;

use crate::{read_reg, write_reg};
use crate::memory_map::stm32f1::AHB_RCC;
use crate::i2c::stm32f1::I2cId;

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

/// Represents a RCC peripheral
pub struct Rcc;

impl Rcc {
    /// Enable the specified port clock
    pub fn enable_gpio_port_clock(port: GpioPort) -> () {
        let bit = Self::get_bit(port);
        let previous_value = read_reg(AHB_RCC + RCC_APB2ENR);
        write_reg(AHB_RCC + RCC_APB2ENR, previous_value | 1 << bit);
    }

    /// Enable the specified I2C
    pub fn enable_i2c(i2c_id: I2cId) -> () {
        let value = match i2c_id {
            I2cId::I2c1 => 1 << 21,
            I2cId::I2c2 => 1 << 22,
        };

        write_reg(AHB_RCC + RCC_APB1ENR, read_reg(AHB_RCC + RCC_APB1ENR) | value);
    }

    /// Disable the clock of the specified GPIO port
    pub fn disable_gpio_port_clock(port: GpioPort) -> () {
        let bit = Self::get_bit(port);
        let previous_value = read_reg(AHB_RCC + RCC_APB2ENR);
        write_reg(AHB_RCC + RCC_APB2ENR, previous_value & !(1 << bit));
    }

    /// Get the bit position of the specified GPIO port
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