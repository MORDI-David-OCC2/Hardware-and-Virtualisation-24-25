/* Linker script for the STM32F103C8T6, loaded by link.x defined in cortex-m-rt */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}
