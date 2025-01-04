//! Modern HAL library for non-modern hardware
//! 
//! [Rust HAL](https://github.com/MORDI-David-OCC2/Hardware-and-Virtualisation-24-25)
//! is a Rust library crate that aims to provide to HAL for several embedded systems,
//! such as the Arduino Nano (Atmega328P) and the STM32F103 (Cortex-M3).

#![no_std]

/// GPIO features
pub mod gpio;

/// Target-specific memory map constants
pub mod memory_map;

/// RCC configuration
pub mod rcc;

/// SPI features
pub mod spi;

/// USART/UART features
pub mod usart;