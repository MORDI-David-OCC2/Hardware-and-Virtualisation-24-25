//! Provides a structure to interact with the Atmega328P I2C peripheral

use crate::{read_reg_8, write_reg_8};

use super::I2cTrait;

//Fréquence du CPU (Atmega328p) en HZ
const F_CPU: u32 = 16_000_000;

// Adresses des registres TWI
const TWBR: u8 = 0xB8;
const TWSR: u8 = 0xB9;
const TWAR: u8 = 0xBA;
const TWDR: u8 = 0xBB;
const TWCR: u8 = 0xBC;

// Bits du registre TWCR
const TWINT: u8 = 1 << 7;
const TWEA:  u8 = 1 << 6;
const TWSTA: u8 = 1 << 5;
const TWSTO: u8 = 1 << 4;
const TWWC:  u8 = 1 << 3;
const TWEN:  u8 = 1 << 2;
const TWIE:  u8 = 1 << 0;

/// Rerpsents an I2C peripheral
pub struct I2c;

impl I2cTrait for I2c {
    fn init(&self) {
        self.twi_init(100_000);
    }

    fn start(&self) -> () {
        write_reg_8(TWCR, TWINT | TWSTA | TWEN);
        while (read_reg_8(TWCR) & TWINT) == 0 {}
    }
    
    // Envoie une condition STOP sur le bus I2C
    fn stop(&self) {
        write_reg_8(TWCR, TWINT | TWSTO | TWEN);
    }
}

impl I2c {
    /// Initialise le TWI en mode maître
    /// Calcul de la valeur TWBR selon la formule du datasheet
    pub fn twi_init(&self, frequency: u32) {
        let prescaler = 1; 
        let twbr_value = ((F_CPU / frequency) - 16) / (2 * prescaler);
    
        write_reg_8(TWSR, 0x00); // Prescaler à 1
        write_reg_8(TWBR, twbr_value as u8); 
        write_reg_8(TWCR, TWEN); // Activer le TWI 
    }
    
    /// Envoie une condition START sur le bus I2C
    /// Retourne true si la condition a été envoyée avec succès.
    pub fn twi_start(&self) -> bool {
        self.start();
        let status = read_reg_8(TWSR) & 0xF8;
        status == 0x08 || status == 0x10
    }
    
    /// Envoie un octet sur le bus I2C.
    pub fn twi_write(&self, data: u8) -> bool {
        write_reg_8(TWDR, data);
        write_reg_8(TWCR, TWINT | TWEN);
        while (read_reg_8(TWCR) & TWINT) == 0 {}
    
        let status = read_reg_8(TWSR) & 0xF8;
    
        status == 0x18 || status == 0x28
    }
    
    /// Lit un octet sur le bus I2C.
    /// Retourne l'octet lu.
    pub fn twi_read(&self, ack: bool) -> u8 {
        let control = if ack { TWINT | TWEN | TWEA } else { TWINT | TWEN };
        write_reg_8(TWCR, control);
        while (read_reg_8(TWCR) & TWINT) == 0 {}
        read_reg_8(TWDR)
    }
}