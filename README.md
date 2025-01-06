# Rust HAL

Ceci est notre HAL, c’est-à-dire une bibliothèque destinée à l’embarquée, qui
permet d’utiliser les fonctionnalités des microcontroleurs de la même manière,
indépendamment de la plateforme utilisé.

Pour l’instant, notre HAL propose les fonctionnalités suivantes :

 - GPIO
 - USART
 - SPI
 - I2C

Des exemples sont fournis pour chaque fonctionnalité. Pour avoir plus d’information, se référer à la partie [*Exemples*](#exemples).

Il est possible d’executer les exemples sans matériel, grâce à QEMU ou à Renode. La procédure à suivre est détaillée dans la section correspondante.

## Exemples

Des exemples sont fournis pour chaque fonctionnalité. Ils se trouvent dans le dossier `examples`.

Pour les compiler, il faut spécifier à Cargo l’exemple et la target désirés.

Par exemple :

    cargo build --example i2c --target thumbv7m-none-eabi

## Supported targets

So far, the following targets are supported:

 - The Atmega328P MCU
 - The STM32F1xxxx MCU family (running on the Cortex-M3 CPU)

### Reasons for choosing the STM32F1 MCU family

The main reason for choosing the STM32F1 family of MCUs is that they run on the Cortex-M3, which is better supported by Rust than the Atmega328P. In addition, they are supported by Renode, an emulation tool that provides a more streamlined workflow and more advanced features than QEMU. For instance, we can define our own peripherals in C#, connect additional devices to any microcontroller, see the connection status and the exchanged data, etc.

## Docker and Dev Containers

The **development environment** used for this project can be run using Docker. Docker is a virtualization software that can be downloaded for free on [their official website](https://docker.com).

Using VS Code, this process can be even more automated! If Docker and VS Code are installed, and the Docker deamon is started (e.g. by running Docker Desktop on Windows) then opening the repository will give a prompt from VS Code offering to switch to the Dev Container. This means it will be possible to run VS Code extensions as well as commands in the terminal in a Linux virtual machine equipped with Rust, Cargo, Rustup, as well as any other dependency!



## Installing the dependencies / running the development environment

The project folder contains a `.devcontainer` folder. This folder, and the file it contains, allows IDEs such as VS Code to open the project in a Docker container (*i.e*. a virtual machine). **This is extremely useful.** It makes it possible to define the OS, the extensions and the programs used to work on the library. They are then automatically installed in the container (the VM) on first opening the folder. The terminal executes commands only in the VM, and this makes it possible to use programs installed in the VM, and the same goes for extensions that rely on specific software being installed (such as Rust-Analyzer requiring Rust).

Here’s a step by step guide with VS Code.

1. First, install Docker. On Windows, you need to download and install Docker Desktop.
2. Run Docker Desktop, so that the Docker daemon is running. You can then minimise the Docker Desktop window or even close the window, Docker will continue running in the background.
![Docker Desktop is running](images/docker_compose.png)
3. Open VS Code and open the project folder.
![Open the project folder in VS Code](images/open_folder.png)
4. You should see the "Reopen in Container" message appear. Click to switch to the Dev Container.
![Cleck on "Reopen in Container"](images/open_in_dev_container.png)

The end result is that VS Code, while running on your host computer, runs as if it was connected to the exact same device that was used to develop this HAL. This guarantees you have the exact same software installed, the same extensions, etc.

If you do not want to use a Dev Container, you can read the `Dockerfile` file and see what software needs to be installed, and how it is installed.


## Test for the Atmega328P

Pour un exemple d’utilisation de l’USART, lancer `cargo run --example usart` depuis un terminal, et `telnet localhost 5678` depuis un autre. Tous les messages que vous enverrez (avec la touche « Entrée ») vous seront retransmis.

Pour tester sur un autre matériel : `cargo run --example usart --target thumbv7m-none-eabi`.

## Test for the STM32F1

The STM32F1 target was chosen because it is supported by Renode. Renode is already installed in the Dev Container. If you are not using the Dev Container you need to install it.

Renode is a very, very useful software for testing embedded software. It is very easy to extend, or to define complex configurations with multiple devices attached to the same MCU.

You first need to build the project. For instance, for the I2C, `cargo build --example i2c --target thumbv7m-none-eabi`.

Then, run Renode:

1. `renode --console`
2. `include @renode/i2c.resc`

We are then able to check that there is a SPI communication between our ficticious STM32F1 and our virtual BME280 sensor!

    22:10:46.9569 [NOISY] i2c1.bme280: Write D0
    22:10:46.9590 [NOISY] i2c1.bme280: Read 60

Here, we queried the sensor for its device ID, and it correctly returned `60`.

To pause the simulation, type `pause` or `p` then press "Enter". To quit renode, type `quit` or `q` then press "Enter".

The same prodecure can be applied for testing the GPIO.

For the USART, the same prodecure can be followed, but once Renode stalls, a second terminal needs to be opened and the following command needs to be run: `telnet localhost 12345`. After that, the message displayed is the one received over USART, and it is possible to send letters by typing a letter.

## Documentation

Le code est commenté. Pour générer la documentation au format HTML, il faut exécuter la commande `cargo doc` depuis le dossier racine du répertoire Git.

Elle peut ensuite être trouvé dans le dossier `./target/avr-atmega328p/doc/tp1/index.html` (selon la target spécifiée).

## Conventions

Par soucis de simplicité, nous désignons (sauf mention contraire) par USART à la fois USART et UART.

## Organisation

Le package contient une crate qui fournit un hal pour l’Atmega328P.

L’organisation est la suivante :

 - `.cargo/config.toml`, `Cargo.toml` et `rust-toolchain.toml` : contient la configuration automatique pour compiler et lancer le programme (choix de la toolchain, manière de lancer le programme, target, `core`…). **Pour changer la target par défaut**, il faut se rendre dans `.cargo/config.toml` and commenter / décommenter la ligne correspondante dans `[build]`.
 - `.devcontainer`: contient la configuration permettant à l’IDE, par exemple VS Code, d’ouvrir le projet dans une machine virtuelle, permettant d’utiliser une configuration identique définie par le projet, avec les bons logiciels installés, les bonnes extensions, et la possibilité d’exécuter les commandes dans l’environnement virtuel.
 - `.vscode`: contient la configuration recommandée pour VS Code pour le projet.
 - `examples/` : montre un exemple de l’utilisation d’USART, de SPI, d’I2C, de GPIO, avec notre bibliothèque.
 - `images/`: images pour le README.
 - `renode/`: platform descriptions and scripts for testing for the STM32F1 with Renode
 - `resources`: datasheets, referenc manuals
 - `src` : contient notre HAL sous forme de Crate library.
 - `avr-atmega328p.json` : contient la configuration personnalisée que nous utilisons pour l’Atmega328P.
 - `Dockerfile`, `docker-compose.yml`: used for building the Dev Container
 - `memory.x`: used by the linker for compiling for the STM32F1


## Corrections

[CORRECTION USART] (Don't hesitate to remove this part)
Very complete project. It is nice that you used traits.
You could try implementing other mode, like asynchrone double speed for the atmega for example.

[CORRECTION SPI] (Don't hesitate to remove this part)
Still very complete. 
You could eventually add some element (this is just some ideas to make it even more perfect, nothing mandatory of course) :
- Abstract your features even more, to support LSB or MSB first data transfer for example.
- Add some safety, for example, you could ensure the master/controler correctly set the clock before enabling the different slave/peripheral.

## Auteurs

- Kaci, Inès
- Matthews, Louis-Marie
- Mordi, David