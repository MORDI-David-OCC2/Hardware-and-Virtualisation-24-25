use crate::memory_map::stm32f1;
use crate::{read_reg, write_reg};

const USART_SR: u32 = 0x00;
const USART_DR: u32 = 0x04;
const USART_BRR: u32 = 0x08;
const USART_CR1: u32 = 0x0C;
const USART_CR2: u32 = 0x10;
const USART_CR3: u32 = 0x14;
const USART_GTPR: u32 = 0x18;

const APB1_MAX_CLOCK_FREQUENCY: u32 = 36_000_000; // APB2 clock frequency (72 MHz for STM32F103)
const APB2_MAX_CLOCK_FREQUENCY: u32 = 72_000_000; // APB2 clock frequency (72 MHz for STM32F103)
const BAUD_RATE: u32 = 115_200; // Desired baud rate

const REG_CR1_RE: u32 = 2;
const REG_SR_TC: u32 = 6;
const REG_SR_RXNE: u8 = 5;

pub enum UsartPeripheral {
    Usart1,
    Usart2,
    Usart3,
    Uart4,
    Uart5,
}

use crate::usart::UsartTrait;

pub struct Usart {
    pub peripheral: UsartPeripheral,
    pub use_9_bit_words: bool,
}

impl Usart {
    pub unsafe fn is_ready(&self) -> bool {
        let tc = core::ptr::read_volatile((self.get_peripheral_offset() + USART_SR) as *mut u32) >> REG_SR_TC & 1;
        1 == tc
    }

    pub unsafe fn set_listening_status(&self, enable_listening: bool) -> () {
        let address = self.get_peripheral_offset() + USART_CR1;
        let value = core::ptr::read_volatile(address as *const u32);
        match enable_listening {
            true => core::ptr::write_volatile(address as *mut u32, value | 1 << REG_CR1_RE),
            false => core::ptr::write_volatile(address as *mut u32, value & !(1 << REG_CR1_RE)),
        }
    }

    fn get_peripheral_offset(&self) -> u32 {
        match self.peripheral {
            UsartPeripheral::Usart1 => stm32f1::APB2_USART1,
            UsartPeripheral::Usart2 => stm32f1::APB1_USART2,
            UsartPeripheral::Usart3 => stm32f1::APB1_USART3,
            UsartPeripheral::Uart4 => stm32f1::APB1_UART4,
            UsartPeripheral::Uart5 => stm32f1::APB1_UART5,
        }
    }
}

impl UsartTrait for Usart {
    /// Initialize the USART/UART peripheral
    /// 
    /// This sets the UE and M bit to enable the peripheral and define the word
    /// length to 8 bit.
    /// 
    /// # Todo
    /// 
    /// [x] Implement method
    /// [x] Make word length configurable
    /// [ ] Set the number of stop bits
    unsafe fn init(&self) -> () {
        let address = self.get_peripheral_offset() + USART_CR1;
        let value = core::ptr::read_volatile(address as *mut u32);
        core::ptr::write_volatile(address as *mut u32, value | 0b10 + (self.use_9_bit_words as u32) << 12);

        // Use clock frequency of bus to which the USART peripheral is attached
        let usart_div = match self.peripheral {
            UsartPeripheral::Usart1 => APB2_MAX_CLOCK_FREQUENCY,
            _ => APB1_MAX_CLOCK_FREQUENCY,
        } / BAUD_RATE;

        core::ptr::write_volatile((self.get_peripheral_offset() + USART_BRR) as *mut u32, usart_div);
    }
    
    // Wait until the TX buffer is cleared, then send the byte
    unsafe fn transmit_byte(&self, byte: u8) {
        unsafe {
            while !self.is_ready() {}
            write_reg(self.get_peripheral_offset() + USART_DR, byte as u32);
        }
    }
    
    unsafe fn receive_byte(&self) -> u8 {
        // Wait until the receive FIFO is not empty
        while (read_reg(self.get_peripheral_offset() + USART_SR) >> REG_SR_RXNE) & 1 != 1 {}
    
        // Read the received byte
        read_reg(self.get_peripheral_offset() + USART_DR) as u8
    }

    /// Envoie d'une chaîne de caractère via l'USART 
    unsafe fn send_message(&self, s: &str) {
        s.as_bytes().iter().for_each(|b| self.transmit_byte(*b));
    }
}