pub mod atmega328p;
pub mod stm32f1;

pub trait GpioTrait {
    fn set_pin_output(&self, pin: u8) -> ();
    // La fonction prend en argument un numéro de broche du port B
    // Lecture de la valeur actuelle de la broche et modification du bit correspondant à la broche donnée
    // Le bit est mis à 1 ce qui configure la broche en sortie 
    // Maj du registre DDRB 
    
    /// Fonction pour configurer une broche comme entrée
    fn set_pin_input(&self, pin: u8) -> ();
    
    /// Fonction pour mettre une broche à l'état haut 
    fn set_pin_high(&self, pin: u8) -> ();
    
    /// Fonction pour mettre une broche à l'état bas 
    fn set_pin_low(&self, pin: u8) -> ();
    
    /// Fonction pour lire l'état d'une broche 
    fn read_pin(&self, pin: u8) -> bool;
}

#[cfg(target_arch = "avr")]
pub use atmega328p::Gpio as Gpio;

#[cfg(target_arch = "arm")]
pub use stm32f1::{
    Gpio as Gpio,
    GpioPort as GpioPort,
    PinOutputCnf,
    PinOutputMode,
};
