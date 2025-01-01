pub const SYSTEM_CLOCK: u32 = 50_000_000; // Example system clock (50 MHz)
pub const BAUD_RATE: u32 = 9_600;          // Desired baud rate

const USART1: u32 = 0x4001_3800;
const USART2: u32 = 0x4000_4400;
const USART3: u32 = 0x4000_4800;
const UART4: u32 = 0x4000_4C00;
const UART5: u32 = 0x4000_5000;

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

use crate::usart::UsartTrait;

pub struct Usart {
    base_address: u32,
    is_uart_only: bool,
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

    fn initialize(&self) {

        // Disable UART0 before configuring
        write_reg(UART0_BASE + UART_CTL_OFFSET, 0);
    
        // Calculate and set the baud rate divisors
        let ibrd = SYSTEM_CLOCK / (16 * BAUD_RATE);
        let fbrd = (((SYSTEM_CLOCK % (16 * BAUD_RATE)) * 64 + BAUD_RATE / 2) / BAUD_RATE) as u32;
    
        write_reg(UART0_BASE + UART_IBRD_OFFSET, ibrd);
        write_reg(UART0_BASE + UART_FBRD_OFFSET, fbrd);
    
        // Configure the UART line control for 8 bits, no parity, 1 stop bit (8N1)
        write_reg(UART0_BASE + UART_LCRH_OFFSET, 0x60);
    
        // Enable UART, transmit, and receive
        write_reg(UART0_BASE + UART_CTL_OFFSET, 0x301);
    }
    
    fn transmit_byte(&self, byte: u8) {
        // Wait until the transmit FIFO is not full
        while read_reg(UART0_BASE + UART_FR_OFFSET) & UART_FR_TXFF != 0 {}
    
        // Send the byte
        write_reg(UART0_BASE + UART_DR_OFFSET, byte as u32);
    }
    
    fn receive_byte(&self) -> u8 {
        // Wait until the receive FIFO is not empty
        while read_reg(UART0_BASE + UART_FR_OFFSET) & UART_FR_RXFE != 0 {}
    
        // Read the received byte
        read_reg(UART0_BASE + UART_DR_OFFSET) as u8
    }

    /// Envoie d'une chaîne de caractère via l'USART 
    fn send_message(&self, s: &str) {
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