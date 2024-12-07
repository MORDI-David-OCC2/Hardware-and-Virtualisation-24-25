use core::ptr;

const DDR_SPI: *mut u8 = 0x24 as *mut u8; // Adresse du registre DDRB
const SPCR: *mut u8 = 0x4C as *mut u8; // Adresse du registre SPCR
const SPSR: *mut u8 = 0x4D as *mut u8; // Adresse du registre SPSR
const SPDR: *mut u8 = 0x4E as *mut u8; // Adresse du registre SPDR

const DD_MOSI: u8 = 3; // MOSI sur PB3
const DD_MISO: u8 = 4; // MISO sur PB4
const DD_SCK: u8 = 5;  // SCK sur PB5

pub struct Spi; 

impl super::Spi for Spi {
    fn init_master() {
        unsafe {
            // Configurer MOSI et SCK comme sorties
            let ddr = ptr::read_volatile(DDR_SPI);
            ptr::write_volatile(DDR_SPI, ddr | (1 << DD_MOSI) | (1 << DD_SCK));
            // Activer SPI en mode maître avec fck/16
            ptr::write_volatile(SPCR, (1 << 6) | (1 << 4) | (1 << 0));
        }
    }

    fn init_slave() {
        unsafe {
            // Configurer MISO comme sortie
            let ddr = ptr::read_volatile(DDR_SPI);
            ptr::write_volatile(DDR_SPI, ddr | (1 << DD_MISO));
            // Activer SPI
            ptr::write_volatile(SPCR, 1 << 6);
        }
    }

    fn transmit(data: u8) {
        unsafe {
            // Écrire les données dans SPDR
            ptr::write_volatile(SPDR, data);
            // Attendre que la transmission soit terminée
            while (ptr::read_volatile(SPSR) & (1 << 7)) == 0 {}
        }
    }

    fn receive() -> u8 {
        unsafe {
            // Attendre que la réception soit terminée
            while (ptr::read_volatile(SPSR) & (1 << 7)) == 0 {}
            // Lire les données reçues
            ptr::read_volatile(SPDR)
        }
    }
}