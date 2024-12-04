#!/bin/sh

# Move to the home directory
cd ~

# Install avr-gcc
wget https://ww1.microchip.com/downloads/aemDocuments/documents/DEV/ProductDocuments/SoftwareTools/avr8-gnu-toolchain-3.7.0.1796-linux.any.x86_64.tar.gz
gunzip -d avr8-gnu-toolchain-3.7.0.1796-linux.any.x86_64.tar.gz
tar -xvf avr8-gnu-toolchain-3.7.0.1796-linux.any.x86_64.tar
mv avr8-gnu-toolchain-3.7.0.1796-linux.any.x86_64 /opt/
echo "export PATH=\"/opt/avr8-gnu-toolchain-3.7.0.1796-linux.any.x86_64/bin:PATH\"" >> .bashrc
source .bashrc

# Install QEMU
apt install qemu-system-avr -y