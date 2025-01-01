FROM rust:latest

# Install QEMU for AVR systems
RUN apt update && apt install qemu-system-avr -y

# Install avr-gcc
RUN wget https://ww1.microchip.com/downloads/aemDocuments/documents/DEV/ProductDocuments/SoftwareTools/avr8-gnu-toolchain-3.7.0.1796-linux.any.x86_64.tar.gz -P /tmp
RUN tar -xvf /tmp/avr8-gnu-toolchain-3.7.0.1796-linux.any.x86_64.tar.gz -C /opt
ENV PATH="$PATH:/opt/avr8-gnu-toolchain-linux_x86_64/bin"

# Install telnet
RUN apt install telnet -y

# Install Renode
RUN wget https://github.com/renode/renode/releases/download/v1.15.3/renode-1.15.3.linux-portable-dotnet.tar.gz -P /tmp
RUN tar -xf /tmp/renode-1.15.3.linux-portable-dotnet.tar.gz -C /opt
ENV PATH="$PATH:/opt/renode_1.15.3-dotnet_portable"

# Install QEMU for ARM systems
RUN apt install qemu-system-arm -y