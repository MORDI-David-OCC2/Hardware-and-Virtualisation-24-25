#![no_std]
#![no_main]

// use core::ptr;
use panic_halt as _;    // Panic handler: halt on panic
use stm32f1::stm32f103;
use cortex_m_semihosting::hprintln;
use tp1::gpio;

// // Base addresses for peripherals
// const RCC_BASE: u32 = 0x40021000; // RCC base address
// const GPIOB_BASE: u32 = 0x40010C00; // GPIOB base address
// const I2C1_BASE: u32 = 0x40005400; // I2C1 base address

// // Offsets for RCC registers
// const RCC_APB2ENR_OFFSET: u32 = 0x18; // APB2 peripheral clock enable register
// const RCC_APB1ENR_OFFSET: u32 = 0x1C; // APB1 peripheral clock enable register

// // Offsets for GPIOB registers
// const GPIOB_CRH_OFFSET: u32 = 0x04;
// const GPIOB_CRL_OFFSET: u32 = 0x00;

// // Offsets for I2C registers
// const I2C_CR1_OFFSET: u32 = 0x00;
// const I2C_CR2_OFFSET: u32 = 0x04;
// const I2C_OAR1_OFFSET: u32 = 0x08;
// const I2C_CCR_OFFSET: u32 = 0x1C;
// const I2C_TRISE_OFFSET: u32 = 0x20;
// const I2C_SR1_OFFSET: u32 = 0x14;
// const I2C_DR_OFFSET: u32 = 0x10;

// // Constants
// const APB1_CLOCK: u32 = 36_000_000; // APB1 clock frequency (36 MHz for STM32F103)
// const I2C_SPEED: u32 = 100_000; // 100 kHz standard mode

// // Entry point
// #[cortex_m_rt::entry]
// fn main() -> ! {
//     // 1. Enable GPIOB and I2C1 clocks
//     unsafe {
//         let rcc_apb2enr = (RCC_BASE + RCC_APB2ENR_OFFSET) as *mut u32;
//         ptr::write_volatile(rcc_apb2enr, ptr::read_volatile(rcc_apb2enr) | (1 << 3)); // Enable GPIOB clock

//         let rcc_apb1enr = (RCC_BASE + RCC_APB1ENR_OFFSET) as *mut u32;
//         ptr::write_volatile(rcc_apb1enr, ptr::read_volatile(rcc_apb1enr) | (1 << 21)); // Enable I2C1 clock
//     }

//     // 2. Configure PB6 (SCL) and PB7 (SDA) as Alternate Function Open-Drain
//     unsafe {
//         let gpiob_crl = (GPIOB_BASE + GPIOB_CRL_OFFSET) as *mut u32;
//         let mut config = ptr::read_volatile(gpiob_crl);
//         config &= !(0xFF << 24); // Clear configuration for PB6 and PB7
//         config |= 0b1111_1111 << 24; // Set PB6 and PB7 as AF Open-Drain, 50 MHz
//         ptr::write_volatile(gpiob_crl, config);
//     }

//     // 3. Configure I2C1
//     unsafe {
//         // Disable I2C1 during configuration
//         let i2c_cr1 = (I2C1_BASE + I2C_CR1_OFFSET) as *mut u32;
//         ptr::write_volatile(i2c_cr1, 0);

//         // Set I2C1 clock frequency (36 MHz APB1 clock)
//         let i2c_cr2 = (I2C1_BASE + I2C_CR2_OFFSET) as *mut u32;
//         ptr::write_volatile(i2c_cr2, 36); // CR2 = APB1 clock in MHz

//         // Set clock control for 100 kHz I2C speed
//         let i2c_ccr = (I2C1_BASE + I2C_CCR_OFFSET) as *mut u32;
//         ptr::write_volatile(i2c_ccr, APB1_CLOCK / (I2C_SPEED * 2)); // Standard mode, T_high/T_low = 50%

//         // Set rise time (max rise time for 100 kHz is 1000 ns)
//         let i2c_trise = (I2C1_BASE + I2C_TRISE_OFFSET) as *mut u32;
//         ptr::write_volatile(i2c_trise, 37); // 1000 ns / (1 / 36 MHz) + 1

//         // Enable I2C1
//         ptr::write_volatile(i2c_cr1, 1); // Enable I2C1
//     }

//     // 4. Transmit data
//     loop {
//         send_i2c(0x3C, &[0x00, 0xA5]); // Send data to device with address 0x3C
//         delay_ms(1000); // Wait 1 second
//     }
// }

// // Function to send data over I2C1
// fn send_i2c(address: u8, data: &[u8]) {
//     unsafe {
//         let i2c_cr1 = (I2C1_BASE + I2C_CR1_OFFSET) as *mut u32;
//         let i2c_sr1 = (I2C1_BASE + I2C_SR1_OFFSET) as *mut u32;
//         let i2c_dr = (I2C1_BASE + I2C_DR_OFFSET) as *mut u32;

//         // Generate START condition
//         ptr::write_volatile(i2c_cr1, ptr::read_volatile(i2c_cr1) | (1 << 8)); // Set START bit
//         while ptr::read_volatile(i2c_sr1) & (1 << 0) == 0 {} // Wait for SB (Start Bit) flag

//         // Send slave address
//         ptr::write_volatile(i2c_dr, ((address << 1) & 0xFE) as u32); // Write address with write bit (0)
//         while ptr::read_volatile(i2c_sr1) & (1 << 1) == 0 {} // Wait for ADDR (Address Sent) flag
//         let _ = ptr::read_volatile(i2c_sr1); // Clear ADDR flag by reading SR1 and SR2
//         let _ = ptr::read_volatile((I2C1_BASE + 0x18) as *mut u32);

//         // Send data
//         for &byte in data {
//             while ptr::read_volatile(i2c_sr1) & (1 << 7) == 0 {} // Wait for TXE (Transmit Empty) flag
//             ptr::write_volatile(i2c_dr, byte as u32); // Write data byte
//         }

//         // Wait for transmission to complete
//         while ptr::read_volatile(i2c_sr1) & (1 << 2) == 0 {} // Wait for BTF (Byte Transfer Finished) flag

//         // Generate STOP condition
//         ptr::write_volatile(i2c_cr1, ptr::read_volatile(i2c_cr1) | (1 << 9)); // Set STOP bit
//     }
// }

use core::ptr;
use tp1::rcc::Rcc;
use tp1::i2c::{I2c, I2cTrait};

#[cfg(target_arch="arm")]
use tp1::i2c::stm32f1::I2cId;

// Base addresses for peripherals
const RCC_BASE: u32 = 0x40021000; // RCC base address
const GPIOB_BASE: u32 = 0x40010C00; // GPIOB base address
const I2C1_BASE: u32 = 0x40005400; // I2C1 base address

// RCC offsets
const RCC_APB2ENR_OFFSET: u32 = 0x18;
const RCC_APB1ENR_OFFSET: u32 = 0x1C;

// GPIOB offsets
const GPIOB_CRL_OFFSET: u32 = 0x00;

// I2C offsets
const I2C_CR1_OFFSET: u32 = 0x00;
const I2C_CR2_OFFSET: u32 = 0x04;
const I2C_OAR1_OFFSET: u32 = 0x08;
const I2C_CCR_OFFSET: u32 = 0x1C;
const I2C_TRISE_OFFSET: u32 = 0x20;
const I2C_DR_OFFSET: u32 = 0x10;
const I2C_SR1_OFFSET: u32 = 0x14;
const I2C_SR2_OFFSET: u32 = 0x18;

// BME280 I2C address
const BME280_ADDR: u8 = 0x76; // Default address (can also be 0x77)

// Entry point
#[cortex_m_rt::entry]
fn main() -> ! {
    // 1. Initialize GPIOB and I2C1
    let gpio_b = gpio::Gpio{port: gpio::GpioPort::B};
    unsafe {
        gpio_b.init();
        gpio_b.set_pin_to_output(gpio::Pin::Pin6, gpio::PinOutputCnf::AlternateFunctionOutputOpenDrain, gpio::PinOutputMode::MaxSpeed2MHZ);
        gpio_b.set_pin_to_output(gpio::Pin::Pin7, gpio::PinOutputCnf::AlternateFunctionOutputOpenDrain, gpio::PinOutputMode::MaxSpeed2MHZ);
    }

    #[cfg(target_arch = "arm")]
    let i2c1 = I2c{
        id: I2cId::I2c1,
    };
    #[cfg(target_arch = "avr")]
    todo!();

    i2c1.init();

    // 2. Read BME280 chip ID as a test
    let chip_id = i2c1.read_register(BME280_ADDR, 0xD0); // 0xD0 is the "chip ID" register
    // hprintln!("BME280 Chip ID: {:#X}", chip_id);

    loop {
        // Example: Read temperature and other sensor data here
        let temp_raw = i2c1.read_register_16(BME280_ADDR, 0xFA); // Temperature MSB starts at 0xFA

        delay_ms(1000); // Wait 1 second
    }
}

// Delay function (busy wait)
// TODO Remove
fn delay_ms(count: u32) {
    for _ in 0..count*8000 {
        // Simple busy-wait loop
        unsafe { core::arch::asm!("nop") }
    }
}