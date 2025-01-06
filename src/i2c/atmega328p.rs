//! Provides a structure to interact with the Atmega328P SPI peripheral

use core::ptr;

//Fréquence du CPU ( Atmega328p) en HZ
const F_CPU: u32 = 16_000_000;

// Adresses des registres TWI
const TWBR: *mut u8 = 0xB8 as *mut u8;
const TWSR: *mut u8 = 0xB9 as *mut u8;
const TWAR: *mut u8 = 0xBA as *mut u8;
const TWDR: *mut u8 = 0xBB as *mut u8;
const TWCR: *mut u8 = 0xBC as *mut u8;

// Bits du registre TWCR
const TWINT: u8 = 1 << 7;
const TWEA:  u8 = 1 << 6; 
const TWSTA: u8 = 1 << 5; 
const TWSTO: u8 = 1 << 4; 
const TWWC:  u8 = 1 << 3; 
const TWEN:  u8 = 1 << 2; 
const TWIE:  u8 = 1 << 0;



// Initialise le TWI en mode maître
// Calcul de la valeur TWBR selon la formule du datasheet
pub fn twi_init(frequency: u32) {
    let prescaler = 1; 
    let twbr_value = ((F_CPU / frequency) - 16) / (2 * prescaler);

    unsafe {
        ptr::write_volatile(TWSR, 0x00); // Prescaler à 1
        ptr::write_volatile(TWBR, twbr_value as u8); 
        ptr::write_volatile(TWCR, TWEN); // Activer le TWI 
    }
}

// Envoie une condition START sur le bus I2C
// Retourne true si la condition a été envoyée avec succès.
pub fn twi_start() -> bool {
    unsafe {
        ptr::write_volatile(TWCR, TWINT | TWSTA | TWEN);
        while (ptr::read_volatile(TWCR) & TWINT) == 0 {}
        let status = ptr::read_volatile(TWSR) & 0xF8;
        status == 0x08 || status == 0x10
    }
}

// Envoie une condition STOP sur le bus I2C
pub fn twi_stop() {
    unsafe {
        ptr::write_volatile(TWCR, TWINT | TWSTO | TWEN);
    }
}

// Envoie un octet sur le bus I2C.
pub fn twi_write(data: u8) -> bool {
    unsafe {
        ptr::write_volatile(TWDR, data);
        ptr::write_volatile(TWCR, TWINT | TWEN);
        while (ptr::read_volatile(TWCR) & TWINT) == 0 {}

        let status = ptr::read_volatile(TWSR) & 0xF8;
    
        status == 0x18 || status == 0x28
    }
}

// Lit un octet sur le bus I2C.
//Retourne l'octet lu.
pub fn twi_read(ack: bool) -> u8 {
    unsafe {
        let control = if ack { TWINT | TWEN | TWEA } else { TWINT | TWEN };
        ptr::write_volatile(TWCR, control);
        while (ptr::read_volatile(TWCR) & TWINT) == 0 {}
        ptr::read_volatile(TWDR)
    }
}