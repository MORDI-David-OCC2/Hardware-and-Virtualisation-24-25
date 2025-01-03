#![no_std]
#![no_main]

#[cfg(target_arch = "avr")]
use tp1::i2c::atmega328p; // Module spécifique à l'ATmega328P
#[cfg(target_arch = "arm")]
use tp1::i2c::cortex_m3;  // Module spécifique au Cortex-M3

#[cfg(target_arch = "avr")]
use panic_halt as _; // Gestion de panique pour AVR

/// Fonction principale pour les tests
#[no_mangle]
pub extern "C" fn main() -> ! {
    #[cfg(target_arch = "avr")]
    test_atmega328p();

    #[cfg(target_arch = "arm")]
    test_cortex_m3();

    loop {}
}

/// Test pour l'ATmega328P
#[cfg(target_arch = "avr")]
fn test_atmega328p() {
    atmega328p::twi_init(100_000); // Initialise le TWI à 100 kHz

    if atmega328p::twi_start() {
        if atmega328p::twi_write(0x50 << 1) { // Adresse périphérique (écriture)
            atmega328p::twi_write(0x00); // Envoyer un registre
            atmega328p::twi_write(0x42); // Envoyer des données
        }
        atmega328p::twi_stop();
    }
}