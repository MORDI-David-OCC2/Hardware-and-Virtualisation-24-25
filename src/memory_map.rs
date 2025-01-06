//! Provides the addresses of memory-mapped devices peripherals
//! 
//! Unlike other modules of this library, no `use` statement is used because
//! registers are specific to a particular platform. As such, they are meant
//! to be used explicitely.

pub mod stm32f1;