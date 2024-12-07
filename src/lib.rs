//! [Rust HAL](https://github.com/MORDI-David-OCC2/Hardware-and-Virtualisation-24-25)
//! is a Rust library crate that aims to provide to HAL for several embedded systems,
//! such as the Arduino Nano (Atmega328P) and the STM32H7 (Cortex-M7).

#![no_std]

pub mod gpio;
pub mod spi;
pub mod usart;