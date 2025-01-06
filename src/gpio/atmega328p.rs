//! Ce fichier contient les fonctions liées à la gestion des broches GPIO
//! 
//! Toutes les fonctions sont publiques pour permettre au fichier main.rs de les utiliser
//! 
//! # TODO
//! [ ] Create memory map for the Atmega328P
//! [ ] Allow creation of GpioPeripheral for each peripheral of the Atmega328P

use crate::{read_reg_8, write_reg_8};

// Registres pour le port B 
const DDRB: u8 = 0x24;
const PORTB: u8 = 0x25;
const PINB: u8 = 0x23;

pub struct Gpio;

impl super::GpioTrait for Gpio {
    /// Fonction qui configure une broche comme sortie 
    fn set_pin_output(&self, pin: u8) {
        write_reg_8(DDRB, read_reg_8(DDRB) | (1 << pin));
    }
    
    // La fonction prend en argument un numéro de broche du port B
    // Lecture de la valeur actuelle de la broche et modification du bit correspondant à la broche donnée
    // Le bit est mis à 1 ce qui configure la broche en sortie 
    // Maj du registre DDRB 
    /// Fonction pour configurer une broche comme entrée
    fn set_pin_input(&self, pin: u8) {
        write_reg_8(DDRB, read_reg_8(DDRB) & !(1 << pin));
    }
    
    /// Fonction pour mettre une broche à l'état haut 
    fn set_pin_high(&self, pin: u8) {
        write_reg_8(PORTB, read_reg_8(PORTB) | (1 << pin));
    }
    
    /// Fonction pour mettre une broche à l'état bas 
    fn set_pin_low(&self, pin: u8) {
        write_reg_8(PORTB, read_reg_8(PORTB) & !(1 << pin));
    }
    
    /// Fonction pour lire l'état d'une broche 
    fn read_pin(&self, pin: u8) -> bool {
        let value = (read_reg_8(PINB) & (1 << pin)) >> pin & 1;
        1 == value
    }
}

impl Gpio {
    /// Fonction pour inverser l'etat d'une broche 
    fn toggle_pin(&self, pin: u8) {
        write_reg_8(PORTB, read_reg_8(PORTB) ^ (1 << pin));
    }
}