#![no_std]
#![no_main]

use panic_halt as _; // Panic handler: halt on panic

use tp1::gpio;
use tp1::i2c::{I2c, I2cTrait};
use tp1::rcc::Rcc;

#[cfg(target_arch = "avr")]
use tp1::i2c::atmega328p; // Module spécifique à l'ATmega328P

#[cfg(target_arch = "arm")]
use stm32f1::stm32f103; // Provide linking information for the compiler

#[cfg(target_arch = "arm")]
use cortex_m_rt::entry; // Provide entry point

#[cfg(target_arch="arm")]
use tp1::i2c::stm32f1::I2cId;

// BME280 I2C address
const BME280_ADDR: u8 = 0x76; // Default address (can also be 0x77)

#[cfg(target_arch="arm")]
#[entry]
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

#[cfg(target_arch = "avr")]
#[no_mangle]
pub extern "C" fn main() -> ! {
    atmega328p::twi_init(100_000); // Initialise le TWI à 100 kHz

    if atmega328p::twi_start() {
        if atmega328p::twi_write(0x50 << 1) { // Adresse périphérique (écriture)
            atmega328p::twi_write(0x00); // Envoyer un registre
            atmega328p::twi_write(0x42); // Envoyer des données
        }
        atmega328p::twi_stop();
    }

    loop {}
}