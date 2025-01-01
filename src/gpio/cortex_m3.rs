//! Ce fichier contient les fonctions liées à la gestion des broches GPIO
//! Toutes les fonctions sont publiques pour permettre au fichier main.rs de les utiliser

// Load memory map constants
use crate::memory_map::stm32f as map;
use crate::rcc;

// TODO to delete
const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB: *mut u8 = 0x23 as *mut u8;

#[derive(Clone)]
pub enum GpioPort {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

pub enum GpioRegister {
    GPIO_CRL,
    GPIO_CRH,
    GPIO_IDR,
    GPIO_ODR,
    GPIO_BSRR,
    GPIO_BRR,
    GPIO_LCKR,
}

pub enum GpioPin {
    
}

pub struct Gpio {
    pub port: GpioPort,
}

impl Gpio {
    fn get_offset(&self) -> u32 {
        match self.port {
            GpioPort::A => map::APB2_GPIO_PORT_A,
            GpioPort::B => map::APB2_GPIO_PORT_B,
            GpioPort::C => map::APB2_GPIO_PORT_C,
            GpioPort::D => map::APB2_GPIO_PORT_D,
            GpioPort::E => map::APB2_GPIO_PORT_E,
            GpioPort::F => map::APB2_GPIO_PORT_F,
            GpioPort::G => map::APB2_GPIO_PORT_G,
        }
    }

    pub unsafe fn init(&self) -> () {
        rcc::Rcc::enable_gpio_port_clock(self.port.clone());
    }
}   

// TODO create enum for pin number? or validate it in other way? Or make it part of GpioPort?
// TODO Point of BSRR and BRR?

impl super::GpioTrait for Gpio {
    /// Fonction qui configure une broche comme sortie 
    /// todo panic if pin not valid
    unsafe fn set_pin_output(&self, pin: u8) -> () {
        let offset = self.get_offset();
        let register = match pin {
            pin if pin <= 7 => map::GPIO_CRL,
            pin if pin > 7 => map::GPIO_CRH,
            _ => panic!(),
        };
        let first_bit_position = if pin <= 7 { (pin-1)*4 } else { (pin-8)*4 };

        let address = (offset + register) as *mut u32;
        let mut value = core::ptr::read_volatile(address);
        value &= !(0b1111 << first_bit_position); // Clear CNF13[1:0] and MODE13[1:0]
        value |= (0b0010 << first_bit_position);  // Set MODE13 to Output mode, max speed 2 MHz
        core::ptr::write_volatile(address, value);
    }
    
    // La fonction prend en argument un numéro de broche du port B
    // Lecture de la valeur actuelle de la broche et modification du bit correspondant à la broche donnée
    // Le bit est mis à 1 ce qui configure la broche en sortie 
    // Maj du registre DDRB 
    
    /// Fonction pour configurer une broche comme entrée
    unsafe fn set_pin_input(&self, pin: u8) {
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) & !(1 << pin));
    }
    
    /// Fonction pour mettre une broche à l'état haut 
    unsafe fn set_pin_high(&self, pin: u8) {
        let address = (self.get_offset() + map::GPIO_BSRR) as *mut u32;
        core::ptr::write_volatile(address, 1 << pin);
    }
    
    /// Fonction pour mettre une broche à l'état haut 
    unsafe fn set_pin_low(&self, pin: u8) {
        let address = (self.get_offset() + map::GPIO_BSRR) as *mut u32;
        let bit_position = 16 + pin;
        core::ptr::write_volatile(address, 1 << bit_position);
    }
    
    /// Fonction pour inverser l'etat d'une broche 
    unsafe fn toggle_pin(&self, pin: u8) {
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) ^ (1 << pin));
    }
    
    /// Fonction pour lire l'état d'une broche 
    unsafe fn read_pin(&self, pin: u8) -> u8 {
        (core::ptr::read_volatile(PINB) & (1 << pin)) >> pin
    }
}
