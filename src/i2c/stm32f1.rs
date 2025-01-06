use super::I2cTrait;

use crate::memory_map::stm32f1::APB1_I2C1;
use crate::memory_map::stm32f1::APB1_I2C2;
use crate::{read_reg, write_reg};

const I2C_CR1: u32 = 0x00;
const I2C_CR2: u32 = 0x04;
const I2C_OAR1: u32 = 0x08;
const I2C_OAR2: u32 = 0x0C;
const I2C_DR: u32 = 0x10;
const I2C_SR1: u32 = 0x14;
const I2C_SR2: u32 = 0x18;
const I2C_CCR: u32 = 0x1C;
const I2C_TRISE: u32 = 0x20;

pub enum I2cId {
    I2c1,
    I2c2,
}

pub struct I2c {
    pub id: I2cId,
}

impl I2cTrait for I2c {
    ///
    /// # todo
    /// [ ] Enable I2C in Clock
    fn init(&self) -> () {
        // @todo Enable I2C clock
    
        // Disable I2C1 during configuration
        write_reg(self.get_reg_addr(I2C_CR1), 0);
    
        // Set APB1 clock frequency (36 MHz)
        write_reg(self.get_reg_addr(I2C_CR2), 36);
    
        // Configure for 100 kHz I2C standard mode
        write_reg(self.get_reg_addr(I2C_CCR), 180); // (36 MHz / (100 kHz * 2))
    
        // Configure maximum rise time
        write_reg(self.get_reg_addr(I2C_TRISE), 37); // 1000 ns / (1 / 36 MHz) + 1
    
        // Enable I2C1
        write_reg(self.get_reg_addr(I2C_CR1), 1);
    }

    // Helper functions for I2C communication
    fn start(&self) {
        // Generate START condition
        write_reg(self.get_reg_addr(I2C_CR1), read_reg(self.get_reg_addr(I2C_CR1)) | (1 << 8));

        // Wait for SB flag
        while read_reg(self.get_reg_addr(I2C_SR1)) & (1 << 0) == 0 {}
    }

    fn stop(&self) {
        // Generate STOP condition
        write_reg(self.get_reg_addr(I2C_CR1), read_reg(self.get_reg_addr(I2C_CR1)) | (1 << 9));
    }
}

impl I2c {
    pub fn get_offset(&self) -> u32 {
        match self.id {
            I2cId::I2c1 => APB1_I2C1,
            I2cId::I2c2 => APB1_I2C2,
        }
    }

    pub fn get_reg_addr(&self, reg_offset: u32) -> u32 {
        return self.get_offset() + reg_offset;
    }

    pub fn i2c_write_byte(&self, byte: u8) {
        // Write byte to data register
        write_reg(self.get_reg_addr(I2C_DR), byte as u32);

        // Wait for TXE flag
        while read_reg(self.get_reg_addr(I2C_SR1)) & (1 << 7) == 0 {}
    }

    pub fn i2c_read_byte(&self) -> u8 {
        // Wait for RXNE flag
        while read_reg(self.get_reg_addr(I2C_SR1)) & (1 << 6) == 0 {}

        read_reg(self.get_reg_addr(I2C_SR1)) as u8
    }

    // Function to write to a register
    pub fn write_register(&self, slave_addr: u8, reg: u8, value: u8) {
        self.start();
        self.i2c_write_byte((slave_addr << 1) & 0xFE); // Write mode
        self.i2c_write_byte(reg);
        self.i2c_write_byte(value);
        self.stop();
    }

    // Function to read a register (8-bit data)
    pub fn read_register(&self, slave_addr: u8, reg: u8) -> u8 {
        self.start();
        self.i2c_write_byte((slave_addr << 1) & 0xFE); // Write mode
        self.i2c_write_byte(reg);
        self.start();
        self.i2c_write_byte((slave_addr << 1) | 0x01); // Read mode
        let value = self.i2c_read_byte();
        self.stop();
        value
    }

    // Function to read a 16-bit value
    pub fn read_register_16(&self, slave_addr: u8, reg: u8) -> u16 {
        self.start();
        self.i2c_write_byte((slave_addr << 1) & 0xFE); // Write mode
        self.i2c_write_byte(reg);
        self.start();
        self.i2c_write_byte((slave_addr << 1) | 0x01); // Read mode
        let msb = self.i2c_read_byte();
        let lsb = self.i2c_read_byte();
        self.stop();
        ((msb as u16) << 8) | (lsb as u16)
    }
}