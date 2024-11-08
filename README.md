Ce projet est fait par David Mordi et Inès Kaci etudiants en OCC2 à l'ESILV.
Il contient pour l'instant la première partie de notre HAL : le gpio.

[CORRECTION GPIO] (Don't hesitate to remove this part)
I couldn't compile ! When you make a project for the first time, I recommand you to use the ```cargo new your_project``` command.
You could abstract your register adresses, by putting them outside your function (as constant)

commandes 7/11/24 USART

Terminal 1

commandes pour compiler :

cargo +nightly build -Z build-std=core --target avr-unknown-gnu-atmega328 --release

avr-gcc -mmcu=atmega328p -o output.elf ./target/avr-unknown-gnu-atmega328/release/deps/*.o ./target/avr-unknown-gnu-atmega328/release/deps/*.rlib

C:\WinAVR-20100110\bin\avr-objcopy.exe -O ihex .\output.elf output.hex

Terminal 2 
Commandes :

qemu-system-avr.exe --machine uno -nographic -bios .\output.elf -serial tcp::5678,server=on

Terminal 3 :
Commandes:
telnet localhost 5678