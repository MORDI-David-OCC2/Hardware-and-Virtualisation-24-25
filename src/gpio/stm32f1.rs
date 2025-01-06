//! Ce fichier contient les fonctions liées à la gestion des broches GPIO
//! Toutes les fonctions sont publiques pour permettre au fichier main.rs de les utiliser

// Load memory map constants
use crate::memory_map::stm32f1 as map;
use crate::{rcc, read_reg, write_reg};

const GPIO_CRL: u32 = 0x00;
const GPIO_CRH: u32 = 0x04;
const GPIO_IDR: u32 = 0x08;
const GPIO_ODR: u32 = 0x0C;
const GPIO_BSRR: u32 = 0x10;
const GPIO_BRR: u32 = 0x14;
const GPIO_LCKR: u32 = 0x18;

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

pub enum GpioInput {
    Analog,
    FloatingInput,
    PullUpPullDownInput,
}

pub enum PinOutputCnf {
    GeneralPurposeOutputPushPull,
    GeneralPurposeOutputOpenDrain,
    AlternateFunctionOutputPushPull,
    AlternateFunctionOutputOpenDrain,
}

pub enum PinOutputMode {
    MaxSpeed10MHZ,
    MaxSpeed2MHZ,
    MaxSpeed50MHZ,
}

pub enum Pin {
    Pin0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
    Pin8,
    Pin9,
    Pin10,
    Pin11,
    Pin12,
    Pin13,
    Pin14,
    Pin15,
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

    /// todo panic if pin not valid
    unsafe fn set_pin_mode(&self, pin: u8, set: bool) -> () {
        let offset = self.get_offset();
        let register = match pin {
            pin if pin <= 7 => GPIO_CRL,
            pin if pin > 7 => GPIO_CRH,
            _ => panic!(),
        };
        let first_bit_position = if pin <= 7 { (pin-1)*4 } else { (pin-8)*4 };

        let address = (offset + register) as *mut u32;
        let mut value = core::ptr::read_volatile(address);
        value &= !(0b1111 << first_bit_position); // Clear CNF13[1:0] and MODE13[1:0]
        value |= if set {0b0010 << first_bit_position} else {0b1000 << first_bit_position};  // Set MODE13 to Output mode, max speed 2 MHz
        core::ptr::write_volatile(address, value);
    }

    ///
    /// ## Todo
    /// [ ] Allow editing of multiple pins of the same register at the same time
    /// [ ] Similar function for input
    pub fn set_pin_to_output(&self, pin: Pin, cnf: PinOutputCnf, mode: PinOutputMode) -> () {
        let pin_no = Self::get_pin_no(pin);
        let reg_addr = self.get_offset() + match pin_no {
            pin_no if pin_no <= 7 => GPIO_CRL,
            pin_no if pin_no > 7 => GPIO_CRH,
            _ => panic!(),
        };

        let cnf_binary = match cnf {
            PinOutputCnf::GeneralPurposeOutputPushPull => 0b00,
            PinOutputCnf::GeneralPurposeOutputOpenDrain => 0b01,
            PinOutputCnf::AlternateFunctionOutputPushPull => 0b10,
            PinOutputCnf::AlternateFunctionOutputOpenDrain => 0b11,
        };
        let mode_binary =  match mode {
            PinOutputMode::MaxSpeed10MHZ => 0b01,
            PinOutputMode::MaxSpeed2MHZ => 0b10,
            PinOutputMode::MaxSpeed50MHZ => 0b11,
        };

        let bit_shift = pin_no % 8 * 4;
        let reg_val = read_reg(reg_addr) & !(0b1111 << bit_shift) | mode_binary << bit_shift | cnf_binary << bit_shift + 2;

        write_reg(reg_addr, reg_val);
    }

    pub fn get_pin_no(pin: Pin) -> u8 {
        match pin {
            Pin::Pin0 => 0,
            Pin::Pin1 => 1,
            Pin::Pin2 => 2,
            Pin::Pin3 => 3,
            Pin::Pin4 => 4,
            Pin::Pin5 => 5,
            Pin::Pin6 => 6,
            Pin::Pin7 => 7,
            Pin::Pin8 => 8,
            Pin::Pin9 => 9,
            Pin::Pin10 => 10,
            Pin::Pin11 => 11,
            Pin::Pin12 => 12,
            Pin::Pin13 => 13,
            Pin::Pin14 => 14,
            Pin::Pin15 => 15,
        }
    }
}   

// TODO create enum for pin number? or validate it in other way? Or make it part of GpioPort?
// TODO Point of BSRR and BRR?

impl super::GpioTrait for Gpio {
    /// Fonction qui configure une broche comme sortie 
    

    unsafe fn set_pin_output(&self, pin: u8) -> () {
        self.set_pin_mode(pin, true);
    }
    
    // La fonction prend en argument un numéro de broche du port B
    // Lecture de la valeur actuelle de la broche et modification du bit correspondant à la broche donnée
    // Le bit est mis à 1 ce qui configure la broche en sortie 
    // Maj du registre DDRB 
    
    /// Fonction pour configurer une broche comme entrée
    unsafe fn set_pin_input(&self, pin: u8) {
        self.set_pin_mode(pin, false);
    }
    
    /// Fonction pour mettre une broche à l'état haut 
    unsafe fn set_pin_high(&self, pin: u8) {
        let address = (self.get_offset() + GPIO_BSRR) as *mut u32;
        core::ptr::write_volatile(address, 1 << pin);
    }
    
    /// Fonction pour mettre une broche à l'état haut 
    unsafe fn set_pin_low(&self, pin: u8) {
        let address = (self.get_offset() + GPIO_BSRR) as *mut u32;
        let bit_position = 16 + pin;
        core::ptr::write_volatile(address, 1 << bit_position);
    }
    
    /// Fonction pour lire l'état d'une broche
    unsafe fn read_pin(&self, pin: u8) -> bool {
        let value: u32 = (core::ptr::read_volatile((self.get_offset() + GPIO_ODR) as *mut u32) >> pin ) & 1;
        1 == value
    }
}
