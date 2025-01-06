# !/usr/bin/env -S python3 -m bpython -i
import time
from pyrenode3 import RPath
import System   # import sys
from pyrenode3.wrappers import Analyzer, Emulation, Monitor
from Antmicro.Renode.PlatformDescription.UserInterface import PlatformDescriptionMachineExtensions
from Antmicro.Renode.Peripherals.I2C import BME280 # src/Infrastructure/src/Emulator/Peripherals/Peripherals/I2C/BME280.cs

mach_name = "stm"
e = Emulation()
m = Monitor()
mach = e.add_mach(mach_name)

load_str = """using "platforms/cpus/stm32l072.repl" bme280: I2C.BME280@ i2c1 0x76"""
PlatformDescriptionMachineExtensions.LoadPlatformDescriptionFromString(mach.internal,load_str)
mach.load_elf("https://dl.antmicro.com/projects/renode/b_l072z_lrwan1--zephyr-bme280_test.elf-s_649120-15b7607a51b50245f4500257c871cd754cfeca5a")

BME280_sen=BME280()
Analyzer(mach.sysbus.usart2).Show()
BME280_sen.Temperature = 45.00
BME280_sen.Humidity = 88.00
e.StartAll()
print(f'{BME280_sen.Temperature},{BME280_sen.Humidity},{BME280_sen.Pressure}')
print('Done')
input()
