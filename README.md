# stm32-blue-pill-rust
Rust for STM32 Blue Pill

## From https://docs.rs/cortex-m-quickstart/0.2.7/cortex_m_quickstart/

Nightly Rust toolchain newer than nightly-2018-04-08: 
rustup default nightly

Cargo clone subcommand: 
cargo install cargo-clone

GDB: 
sudo apt-get install gdb-arm-none-eabi (on Ubuntu)

OpenOCD: 
sudo apt-get install OpenOCD (on Ubuntu)

[Optional] ARM linker: 
sudo apt-get install binutils-arm-none-eabi (on Ubuntu)

[Optional] Cargo add subcommand: 
cargo install cargo-edit

-----
## Install prerequisites

1. We assume that you are running Ubuntu 18.04 LTS x86 64-bit on a PC or on Oracle VirtualBox.

1. If you're using Oracle VirtualBox, you may download the preinstalled Ubuntu image from <br>
    https://drive.google.com/drive/folders/1Isf0lAj6otziOXYjjSrfwCW06rQEwzaG?usp=sharing
    
    In VirtualBox, click `File → Import Appliance` and select the downloaded file `padi.ova`. <br>
    Start the `padi` virtual machine and connect with `ssh` to `padi.local`, username `user`, password `password`.

1. Run the following commands on Ubuntu:

    ```bash
    sudo apt update
    sudo apt install build-essential gawk bc libc6-dev:i386 lib32ncurses5-dev
    sudo apt install gcc-arm-none-eabi
    sudo apt install gdb-multiarch
    sudo apt install cutecom
    sudo apt install openocd
    ```

## Download rustl8710 code

1. Run the following command on Ubuntu:

    ```bash
    git clone https://github.com/lupyuen/rustl8710
    ```
    
## Install Rust components

1. On Ubuntu, install `rustup` from https://rustup.rs/ (select default installation):

    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```

1. Run the following commands on Ubuntu:

    ```bash
    source $HOME/.cargo/env
    cd rustl8710
    rustup update
    rustup override set nightly
    rustup component add rust-src
    cargo install xargo
    ```

1. This installs the Rust compiler and libraries, as well as Xargo the Rust cross-compiler. The nightly Rust library build is selected (instead of the stable or beta builds) because Xargo requires it.

