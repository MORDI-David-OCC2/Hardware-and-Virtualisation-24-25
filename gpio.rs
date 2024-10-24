//Ce fichier contient les fonctions liées à la gestion des broches GPIO



// Registres pour le port B 

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB: *mut u8 = 0x23 as *mut u8; 

//Toutes les fonctions son publiques pour permettre au fichier main.rs de les utiliser

// Fonction qui configure une broche comme sortie 


pub unsafe fn set_pin_output(pin: u8) {
    core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | (1 << pin));
}

//La fonction prend en argument un numéro de broche du port B
//Lecture de la valeur actuelle de la broche et modification du bit correspondant à la broche donnée
//Le bit est mis à 1 ce qui configure la broche en sortie 
//Maj du registre DDRB 

// Fonction pour configurer une broche comme entrée
pub unsafe fn set_pin_input(pin: u8) {
    core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) & !(1 << pin));
}

// Fonction pour mettre une broche à l'état haut 
pub unsafe fn set_pin_high(pin: u8) {
    core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) | (1 << pin));
}

// Fonction pour mettre une broche à l'état bas 
pub unsafe fn set_pin_low(pin: u8) {
    core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) & !(1 << pin));
}

// Fonction pour inverser l'etat d'une broche 
pub unsafe fn toggle_pin(pin: u8) {
    core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) ^ (1 << pin));
}

// Fonction pour lire l'état d'une broche 
pub unsafe fn read_pin(pin: u8) -> u8 {
    (core::ptr::read_volatile(PINB) & (1 << pin)) >> pin
}