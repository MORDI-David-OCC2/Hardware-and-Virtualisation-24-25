#[cfg(target_arch = "avr")]
pub mod atmega328p;

#[cfg(target_arch = "arm")]
pub mod cortex_m3;

pub enum GpioPort {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

pub trait GpioTrait {
    unsafe fn set_pin_output(&self, pin: u8) -> ();
    // La fonction prend en argument un numéro de broche du port B
    // Lecture de la valeur actuelle de la broche et modification du bit correspondant à la broche donnée
    // Le bit est mis à 1 ce qui configure la broche en sortie 
    // Maj du registre DDRB 
    
    /// Fonction pour configurer une broche comme entrée
    unsafe fn set_pin_input(pin: u8);
    
    /// Fonction pour mettre une broche à l'état haut 
    unsafe fn set_pin_high(pin: u8);
    
    /// Fonction pour mettre une broche à l'état bas 
    unsafe fn set_pin_low(pin: u8);
    
    /// Fonction pour inverser l'etat d'une broche 
    unsafe fn toggle_pin(pin: u8);
    
    /// Fonction pour lire l'état d'une broche 
    unsafe fn read_pin(pin: u8) -> u8;
}

#[cfg(target_arch = "avr")]
pub use atmega328p::Gpio as Gpio;