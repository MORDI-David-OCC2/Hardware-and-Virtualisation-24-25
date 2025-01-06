//! Modern HAL library for non-modern hardware
//! 
//! [Rust HAL](https://github.com/MORDI-David-OCC2/Hardware-and-Virtualisation-24-25)
//! is a Rust library crate that aims to provide to HAL for several embedded systems,
//! such as the Arduino Nano (Atmega328P) and the STM32F103 (Cortex-M3).

#![no_std]

/// GPIO features
pub mod gpio;

/// I2C
pub mod i2c;

/// Target-specific memory map constants
pub mod memory_map;

/// RCC configuration
pub mod rcc;

/// SPI features
pub mod spi;

/// USART/UART features
pub mod usart;

/// Read 32-bit register with 32-bit address
pub fn read_reg(addr: u32) -> u32 {
    unsafe {
        (addr as *const u32).read_volatile()
    }
}

/// Read 32-bit value at 32-bit address
pub fn write_reg(addr: u32, value: u32) {
    unsafe {
        (addr as *mut u32).write_volatile(value);
    }
}