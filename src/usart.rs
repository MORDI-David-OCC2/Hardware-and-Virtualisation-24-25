//! This module provides a trait for using USART module on different systems.

pub trait Hal {
    fn iniatilize_usart();
}