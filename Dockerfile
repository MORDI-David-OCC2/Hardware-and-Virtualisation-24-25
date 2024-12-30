FROM rust:latest

# Install QEMU
RUN apt update && apt install qemu-system-avr -y

# Install avr-gcc
RUN wget https://ww1.microchip.com/downloads/aemDocuments/documents/DEV/ProductDocuments/SoftwareTools/avr8-gnu-toolchain-3.7.0.1796-linux.any.x86_64.tar.gz -P /tmp
RUN gunzip -d /tmp/avr8-gnu-toolchain-3.7.0.1796-linux.any.x86_64.tar.gz
RUN tar -xvf /tmp/avr8-gnu-toolchain-3.7.0.1796-linux.any.x86_64.tar -C /opt
ENV PATH="$PATH:/opt/avr8-gnu-toolchain-linux_x86_64/bin"

# Install telnet
RUN apt install telnet -y