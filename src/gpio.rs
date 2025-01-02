#[cfg(target_arch = "avr")]
pub mod atmega328p;

#[cfg(target_arch = "arm")]
pub mod cortex_m3;

pub trait GpioTrait {
    unsafe fn set_pin_output(&self, pin: u8) -> ();
    // La fonction prend en argument un numéro de broche du port B
    // Lecture de la valeur actuelle de la broche et modification du bit correspondant à la broche donnée
    // Le bit est mis à 1 ce qui configure la broche en sortie 
    // Maj du registre DDRB 
    
    /// Fonction pour configurer une broche comme entrée
    unsafe fn set_pin_input(&self, pin: u8);
    
    /// Fonction pour mettre une broche à l'état haut 
    unsafe fn set_pin_high(&self, pin: u8);
    
    /// Fonction pour mettre une broche à l'état bas 
    unsafe fn set_pin_low(&self, pin: u8);
    
    /// Fonction pour lire l'état d'une broche 
    unsafe fn read_pin(&self, pin: u8) -> bool;
}

#[cfg(target_arch = "avr")]
pub use atmega328p::Gpio as Gpio;

#[cfg(target_arch = "arm")]
pub use cortex_m3::{Gpio as Gpio, GpioPort as GpioPort};
