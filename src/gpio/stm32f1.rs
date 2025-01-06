//! Ce fichier contient les fonctions liées à la gestion des broches GPIO
//! Toutes les fonctions sont publiques pour permettre au fichier main.rs de les utiliser

// Load memory map constants
use crate::memory_map::stm32f1 as map;
use crate::rcc::stm32f1::Rcc;
use crate::{read_reg, write_reg};

const GPIO_CRL: u32 = 0x00;
const GPIO_CRH: u32 = 0x04;
const GPIO_IDR: u32 = 0x08;
const GPIO_ODR: u32 = 0x0C;
const GPIO_BSRR: u32 = 0x10;
const GPIO_BRR: u32 = 0x14;
const GPIO_LCKR: u32 = 0x18;

/// Defines the existing GPIO ports existing on any STM32F1 device
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

/// Defines the different input configurations
pub enum GpioInput {
    Analog,
    FloatingInput,
    PullUpPullDownInput,
}

/// Defines the different output CNF configurations
pub enum PinOutputCnf {
    GeneralPurposeOutputPushPull,
    GeneralPurposeOutputOpenDrain,
    AlternateFunctionOutputPushPull,
    AlternateFunctionOutputOpenDrain,
}

/// Defines the supported output speeds
pub enum PinOutputMode {
    MaxSpeed10MHZ,
    MaxSpeed2MHZ,
    MaxSpeed50MHZ,
}

/// Represents a GPIO port of a STM32F1 device
pub struct Gpio {
    /// The port managed by the instance
    pub port: GpioPort,
}

impl Gpio {
    /// Get the address in memory of the peripheral
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

    /// Initialize the peripheral
    pub unsafe fn init(&self) -> () {
        Rcc::enable_gpio_port_clock(self.port.clone());
    }

    /// Sets the pin mode
    /// 
    /// todo panic if pin not valid
    fn set_pin_mode(&self, pin: u8, set: bool) -> () {
        let offset = self.get_offset();
        let register = match pin {
            pin if pin <= 7 => GPIO_CRL,
            pin if pin > 7 => GPIO_CRH,
            _ => panic!(),
        };
        let first_bit_position = if pin <= 7 { (pin-1)*4 } else { (pin-8)*4 };

        let address = offset + register;
        let mut value = read_reg(address.clone());
        value &= !(0b1111 << first_bit_position); // Clear CNF13[1:0] and MODE13[1:0]
        value |= if set {0b0010 << first_bit_position} else {0b1000 << first_bit_position};  // Set MODE13 to Output mode, max speed 2 MHz
        write_reg(address, value);
    }

    /// Sets the pin as output
    /// 
    /// ## Todo
    /// [ ] Allow editing of multiple pins of the same register at the same time
    /// [ ] Similar function for input
    pub fn set_pin_to_output(&self, pin: u8, cnf: PinOutputCnf, mode: PinOutputMode) -> () {
        let reg_addr = self.get_offset() + match pin {
            pin_no if pin_no <= 7 => GPIO_CRL,
            pin_no if pin_no >= 8 && pin_no <= 15 => GPIO_CRH,
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

        let bit_shift = pin % 8 * 4;
        let reg_val = read_reg(reg_addr) & !(0b1111 << bit_shift) | cnf_binary << bit_shift + 2 | mode_binary << bit_shift;

        write_reg(reg_addr, reg_val);
    }
}   

impl super::GpioTrait for Gpio {
    /// Fonction qui configure une broche comme sortie 
    fn set_pin_output(&self, pin: u8) -> () {

        self.set_pin_to_output(pin, PinOutputCnf::AlternateFunctionOutputPushPull, PinOutputMode::MaxSpeed2MHZ);
    }
    
    /// Fonction pour configurer une broche comme entrée
    /// 
    /// La fonction prend en argument un numéro de broche du port B
    /// Le bit est mis à 1 ce qui configure la broche en sortie 
    /// Lecture de la valeur actuelle de la broche et modification du bit correspondant à la broche donnée
    /// Maj du registre DDRB 
    fn set_pin_input(&self, pin: u8) {
        self.set_pin_mode(pin, false);
    }
    
    /// Fonction pour mettre une broche à l'état haut 
    fn set_pin_high(&self, pin: u8) {
        write_reg(self.get_offset() + GPIO_BSRR, 1 << pin);
    }
    
    /// Fonction pour mettre une broche à l'état haut 
    fn set_pin_low(&self, pin: u8) {
        let bit_position = 16 + pin;
        write_reg(self.get_offset() + GPIO_BSRR, 1 << bit_position);
    }
    
    /// Fonction pour lire l'état d'une broche
    fn read_pin(&self, pin: u8) -> bool {
        let value: u32 = (read_reg(self.get_offset() + GPIO_ODR) >> pin ) & 1;
        1 == value
    }
}
