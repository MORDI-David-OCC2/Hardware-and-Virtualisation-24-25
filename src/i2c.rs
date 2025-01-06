pub mod stm32f1;

pub trait I2cTrait {
    fn init(&self) -> ();
    fn write_register(&self, slave_addr: u8, reg: u8, value: u8) -> ();
    fn read_register(&self, slave_addr: u8, reg: u8) -> u8;
    fn read_register_16(&self, slave_addr: u8, reg: u8) -> u16;
}

#[cfg(target_arch = "arm")]
pub use stm32f1::I2c as I2c;