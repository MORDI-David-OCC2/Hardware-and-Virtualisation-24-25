//! Ce fichier contient les fonctions liées à la gestion des broches GPIO
//! Toutes les fonctions sont publiques pour permettre au fichier main.rs de les utiliser

// Load memory map constants
pub mod memory_map;

use super::GpioPort;

// TODO to delete
const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB: *mut u8 = 0x23 as *mut u8;

pub enum GpioRegister {
    GPIOx_CRL = 0x00,
    GPIOx_CRH = 0x04,
    GPIOx_IDR = 0x08,
    GPIOx_ODR = 0x0C,
    GPIOx_BSRR = 0x10,
    GPIOx_BRR = 0x14,
    GPIOx_LCKR = 0x18,
}

pub enum GpioPin {
    
}

pub struct Gpio {
    offset: u32,
}

impl Gpio {
    pub fn new(port: GpioPort) -> Self {
        let offset = match port {
            GpioPort::A => memory_map::APB2_GPIO_PORT_A,
            GpioPort::B => memory_map::APB2_GPIO_PORT_B,
            GpioPort::C => memory_map::APB2_GPIO_PORT_C,
            GpioPort::D => memory_map::APB2_GPIO_PORT_D,
            GpioPort::E => memory_map::APB2_GPIO_PORT_E,
            GpioPort::F => memory_map::APB2_GPIO_PORT_F,
            GpioPort::G => memory_map::APB2_GPIO_PORT_G,
        };
        Self {offset}
    }
}

// TODO create enum for pin number? or validate it in other way? Or make it part of GpioPort?
// TODO Point of BSRR and BRR?

impl super::GpioTrait for Gpio {
    /// Fonction qui configure une broche comme sortie 
    unsafe fn set_pin_output(&self, pin: u8) -> ()   {
        let register = match pin {
            pin if pin < 8 => GpioRegister::GPIOx_CRL as u32,
            pin if pin >= 8 => GpioRegister::GPIOx_CRH as u32,
            _ => panic!(),
        };
        let address = (self.offset + register) as *mut u32;
        core::ptr::write_volatile(address, (core::ptr::read_volatile(address) & 0b0000) | (0b0001 << (pin*2)));
    }
    
    // La fonction prend en argument un numéro de broche du port B
    // Lecture de la valeur actuelle de la broche et modification du bit correspondant à la broche donnée
    // Le bit est mis à 1 ce qui configure la broche en sortie 
    // Maj du registre DDRB 
    
    /// Fonction pour configurer une broche comme entrée
    unsafe fn set_pin_input(pin: u8) {
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) & !(1 << pin));
    }
    
    /// Fonction pour mettre une broche à l'état haut 
    unsafe fn set_pin_high(pin: u8) {
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) | (1 << pin));
    }
    
    /// Fonction pour mettre une broche à l'état bas 
    unsafe fn set_pin_low(pin: u8) {
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) & !(1 << pin));
    }
    
    /// Fonction pour inverser l'etat d'une broche 
    unsafe fn toggle_pin(pin: u8) {
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) ^ (1 << pin));
    }
    
    /// Fonction pour lire l'état d'une broche 
    unsafe fn read_pin(pin: u8) -> u8 {
        (core::ptr::read_volatile(PINB) & (1 << pin)) >> pin
    }
}
