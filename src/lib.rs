//! Modern HAL library for non-modern hardware
//! 
//! [Rust HAL](https://github.com/MORDI-David-OCC2/Hardware-and-Virtualisation-24-25)
//! is a HAL currently supporting two platforms:
//! - the Arduino Nano (running on the Atmega328P),
//! - the STM32F103 (running on the Cortex-M3).
//! 
//! # Crate organisation
//! 
//! The crate is organised into **feature** modules, such as:
//! 
//! - GPIO (`gpio.rs`),
//! - I2C (`i2c.rs`)
//! - …
//! 
//! Each feature module defines, in its associated folder, one submodule for
//! each supported platform. We call them "**platform submodules**".
//! 
//! For instance, for GPIO, we find:
//! 
//! - `gpio/atmega328p.rs`
//! - `gpio/stm32f1.rs`
//! 
//! Certain features are platform dependent, and as such the code needs to
//! explicitely specify the platform submodule. For instance:
//! 
//! ```
//! use tp1::gpio::Gpio;
//! let gpio_b = Gpio {
//!     port: gpio::stm32f1::GpioPort::B,
//! };
//! ```
//! 
//! Other features can be accessed directly from the feature module.

#![no_std]
#![allow(dead_code)]
#![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]

pub mod gpio;
pub mod i2c;
pub mod memory_map;
pub mod rcc;
pub mod spi;
pub mod timers;
pub mod usart;

/// Read 32-bit register with 32-bit address
pub fn read_reg(addr: u32) -> u32 {
    unsafe {
        (addr as *const u32).read_volatile()
    }
}

/// Write 32-bit value at 32-bit address
pub fn write_reg(addr: u32, value: u32) {
    unsafe {
        (addr as *mut u32).write_volatile(value);
    }
}

/// Read 8-bit register with 8-bit address
pub fn read_reg_8(addr: u8) -> u8 {
    unsafe {
        (addr as *const u8).read_volatile()
    }
}

/// Write 8-bit value at 8-bit address
pub fn write_reg_8(addr: u8, value: u8) {
    unsafe {
        (addr as *mut u8).write_volatile(value);
    }
}