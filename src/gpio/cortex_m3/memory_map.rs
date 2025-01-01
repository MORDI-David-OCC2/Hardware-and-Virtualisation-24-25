// Memory map of the STM32F1xx
// All reserved peripherals were omitted.

// AHB Peripherals
// AHB Peripherals are peripherals directly connected to the AHB bus.
pub const AHB_FSMC: u32 = 0xA000_0000;
pub const AHB_USB_OTG_FS: u32 = 0x5000_0000;
pub const AHB_ETHERNET: u32 = 0x4002_8000;
pub const AHB_CRC: u32 = 0x4002_3000;
pub const AHB_FLASH_MEMORY_INTERFACE_: u32 = 0x4002_2000;
pub const AHB_RCC: u32 = 0x4002_1000;
pub const AHB_DMA2: u32 = 0x4002_0400;
pub const AHB_DMA1: u32 = 0x4002_0000;
pub const AHB_SDIO: u32 = 0x4001_8000;

// APB2
// APB Peripherals are peripherals directly connected to the AP2 bus, also called
// the high speed APB bus. Its clock frequency is limited to TODO
pub const APB2_TIM11_TIMER: u32 = 0x4001_5400;
pub const APB2_TIM10_TIMER: u32 = 0x4001_5000;
pub const APB2_TIM9_TIMER: u32 = 0x4001_4C00;
pub const APB2_ADC3: u32 = 0x4001_3C00;
pub const APB2_USART1: u32 = 0x4001_3800;
pub const APB2_TIM8_TIMER: u32 = 0x4001_3400;
pub const APB2_SPI1: u32 = 0x4001_3000;
pub const APB2_ADC2: u32 = 0x4001_2800;
pub const APB2_TIM1_TIMER: u32 = 0x4001_2C00;
pub const APB2_GPIO_PORT_G: u32 = 0x4001_2000;
pub const APB2_ADC1: u32 = 0x4001_2400;
pub const APB2_GPIO_PORT_F: u32 = 0x4001_1C00;
pub const APB2_GPIO_PORT_E: u32 = 0x4001_1800;
pub const APB2_GPIO_PORT_D: u32 = 0x4001_1400;
pub const APB2_GPIO_PORT_C: u32 = 0x4001_1000;
pub const APB2_GPIO_PORT_B: u32 = 0x4001_0C00;
pub const APB2_GPIO_PORT_A: u32 = 0x4001_0800;
pub const APB2_EXTI: u32 = 0x4001_0400;
pub const APB2_AFIO: u32 = 0x4001_0000;

// APB1
// APB Peripherals are peripherals directly connected to the AP2 bus, also called
// the low speed APB bus. Its clock frequency is limited to TODO
pub const APB1_DAC: u32 = 0x4000_7400;
pub const APB1_PWR: u32 = 0x4000_7000;
pub const APB1_BXCAN1: u32 = 0x4000_6400;
pub const APB1_BKP: u32 = 0x4000_6C00;
pub const APB1_BXCAN2: u32 = 0x4000_6800;
pub const APB1_SHARED_USB_CAN_SRAM_512_BYTES: u32 = 0x4000_6000;
pub const APB1_I2C2: u32 = 0x4000_5800;
pub const APB1_USB_DEVICE_FS_REGISTERS: u32 = 0x4000_5C00;
pub const APB1_UART5: u32 = 0x4000_5000;
pub const APB1_I2C1: u32 = 0x4000_5400;
pub const APB1_UART4: u32 = 0x4000_4C00;
pub const APB1_USART3: u32 = 0x4000_4800;
pub const APB1_USART2: u32 = 0x4000_4400;
pub const APB1_SPI3_I2S: u32 = 0x4000_3C00;
pub const APB1_SPI2_I2S: u32 = 0x4000_3800;
pub const APB1_IWDG: u32 = 0x4000_3000;
pub const APB1_WWDG: u32 = 0x4000_2C00;
pub const APB1_RTC: u32 = 0x4000_2800;
pub const APB1_TIM14_TIMER: u32 = 0x4000_2000;
pub const APB1_TIM13_TIMER: u32 = 0x4000_1C00;
pub const APB1_TIM7_TIMER: u32 = 0x4000_1400;
pub const APB1_TIM12_TIMER: u32 = 0x4000_1800;
pub const APB1_TIM5_TIMER: u32 = 0x4000_0C00;
pub const APB1_TIM6_TIMER: u32 = 0x4000_1000;
pub const APB1_TIM4_TIMER: u32 = 0x4000_0800;
pub const APB1_TIM3_TIMER: u32 = 0x4000_0400;
pub const APB1_TIM2_TIMER: u32 = 0x4000_0000;