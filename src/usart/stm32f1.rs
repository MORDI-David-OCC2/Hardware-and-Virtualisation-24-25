use crate::memory_map::stm32f1;

const USART_SR: u32 = 0x00;
const USART_DR: u32 = 0x04;
const USART_BRR: u32 = 0x08;
const USART_CR1: u32 = 0x0C;
const USART_CR2: u32 = 0x10;

const UART0_BASE: u32 = 0x4000C000; // Base address for UART0 (check LM3S6965 datasheet for specific addresses)
const UART_DR_OFFSET: u32 = 0x000;  // Data register offset
const UART_FR_OFFSET: u32 = 0x018;  // Flag register offset
const UART_IBRD_OFFSET: u32 = 0x024; // Integer Baud Rate Divisor
const UART_FBRD_OFFSET: u32 = 0x028; // Fractional Baud Rate Divisor
const UART_LCRH_OFFSET: u32 = 0x02C; // Line Control Register
const UART_CTL_OFFSET: u32 = 0x030;  // Control register

const UART_FR_TXFF: u32 = 1 << 5;    // Transmit FIFO Full
const UART_FR_RXFE: u32 = 1 << 4;    // Receive FIFO Empty

const APB1_MAX_CLOCK_FREQUENCY: u32 = 36_000_000; // APB2 clock frequency (72 MHz for STM32F103)
const APB2_MAX_CLOCK_FREQUENCY: u32 = 72_000_000; // APB2 clock frequency (72 MHz for STM32F103)
pub const BAUD_RATE: u32 = 115_200; // Desired baud rate

// const REG_CR1_RE
const REG_SR_TC: u32 = 6;

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
    // fn new(interface_n: u8) -> Self {

    //     let base_address = match interface_n {
    //         1 => USART1,
    //         2 => USART2,
    //         3 => USART3,
    //         4 => UART4,
    //         5 => UART5,
    //         _ => panic!()
    //     };
    //     let uart_only = interface_n > 3;
    //     Self use uart_only {
    //         base_address,
    //         uart_only,
    //     }
    // }

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
        while read_reg(UART0_BASE + UART_FR_OFFSET) & UART_FR_RXFE != 0 {}
    
        // Read the received byte
        read_reg(UART0_BASE + UART_DR_OFFSET) as u8
    }

    /// Envoie d'une chaîne de caractère via l'USART 
    unsafe fn send_message(&self, s: &str) {
        s.as_bytes().iter().for_each(|b| self.transmit_byte(*b));
    }
}

fn write_reg(addr: u32, value: u32) {
    unsafe {
        (addr as *mut u32).write_volatile(value);
    }
}

fn read_reg(addr: u32) -> u32 {
    unsafe {
        (addr as *const u32).read_volatile()
    }
}