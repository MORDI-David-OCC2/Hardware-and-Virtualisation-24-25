# Rust HAL

## USART

Pour un exemple d’utilisation de l’USART, lancer `cargo run --example usart` depuis un terminal, et `telnet localhost 5678` depuis un autre. Tous les messages que vous enverrez (avec la touche « Entrée ») vous seront retransmis.

## Organisation

Le package contient une crate qui fournit un hal pour l’Atmega328P.

L’organisation est la suivante :

 - `.cargo/config.toml`, `Cargo.toml` et `rust-toolchain.toml` : contient la configuration automatique pour compiler et lancer le programme (choix de la toolchain, manière de lancer le programme, target, `core`…)
 - `examples/usart.rs` : montre un exemple de l’utilisation d’USART avec notre bibliothèque.
 - `avr-atmega328p.json` : contient la configuration personnalisée que nous utilisons pour l’Atmega328P.
 - `src` : contient notre HAL sous forme de Crate library.

## À propos

Ce projet est fait par David Mordi, Inès Kaci et Louis-Marie Matthews, étudiants en OCC2 à l'ESILV.

## Instructions de lancement sur Windows

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