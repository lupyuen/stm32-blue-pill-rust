# stm32-blue-pill-rust
Rust for STM32 Blue Pill

## Install Prerequisities

```
sudo apt install pkg-config
sudo apt install cmake
```

## From https://docs.rs/cortex-m-quickstart/0.2.7/cortex_m_quickstart/

```
Nightly Rust toolchain newer than nightly-2018-04-08:
rustup default nightly

Cargo clone subcommand: 
cargo install cargo-clone

GDB: (on Ubuntu)
## Fails: sudo apt install gdb-arm-none-eabi
sudo apt install gdb-multiarch

OpenOCD: (on Ubuntu)
sudo apt install OpenOCD

[Optional] ARM linker: (on Ubuntu)
sudo apt install binutils-arm-none-eabi

[Optional] Cargo add subcommand: 
cargo install cargo-edit
```

Use thumbv7m-none-eabi for ARM Cortex-M3

```
Install the rust-std component for your target, if you haven't done so already

$ rustup target add thumbv7m-none-eabihf

Clone this crate

$ cargo clone cortex-m-quickstart && cd $_

Change the crate name, author and version

$ edit Cargo.toml && head $_
[package]
authors = ["Jorge Aparicio <jorge@japaric.io>"]
name = "demo"
version = "0.1.0"

Specify the memory layout of the target device
NOTE board support crates sometimes provide this file for you (check the crate documentation). If you are using one that does then remove both the memory.x and build.rs files.

$ cat >memory.x <<'EOF'
MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
EOF

Optionally, set a default build target

$ cat >>.cargo/config <<'EOF'
[build]
target = "thumbv7em-none-eabihf"
EOF

Optionally, depend on a device, HAL implementation or a board support crate.

$ # add a device crate, OR
$ cargo add stm32f30x

$ # add a HAL implementation crate, OR
$ cargo add stm32f30x-hal

$ # add a board support crate
$ cargo add f3

Write the application or start from one of the examples

$ rm -r src/* && cp examples/hello.rs src/main.rs

Build the application

$ cargo build --release
$ # sanity check
$ arm-none-eabi-readelf -A target/thumbv7em-none-eabihf/release/demo

Attribute Section: aeabi
File Attributes
  Tag_conformance: "2.09"
  Tag_CPU_arch: v7E-M
  Tag_CPU_arch_profile: Microcontroller
  Tag_THUMB_ISA_use: Thumb-2
  Tag_FP_arch: VFPv4-D16
  Tag_ABI_PCS_GOT_use: direct
  Tag_ABI_FP_denormal: Needed
  Tag_ABI_FP_exceptions: Needed
  Tag_ABI_FP_number_model: IEEE 754
  Tag_ABI_align_needed: 8-byte
  Tag_ABI_align_preserved: 8-byte, except leaf SP
  Tag_ABI_HardFP_use: SP only
  Tag_ABI_VFP_args: VFP registers
  Tag_ABI_optimization_goals: Aggressive Speed
  Tag_CPU_unaligned_access: v6
  Tag_FP_HP_extension: Allowed
  Tag_ABI_FP_16bit_format: IEEE 754

NOTE By default Cargo will use the LLD linker shipped with the Rust toolchain. If you encounter any linking error try to switch to the GNU linker by modifying the .cargo/config file as shown below:

 runner = 'arm-none-eabi-gdb'
 rustflags = [
   "-C", "link-arg=-Tlink.x",
-  "-C", "linker=lld",
-  "-Z", "linker-flavor=ld.lld",
-  # "-C", "linker=arm-none-eabi-ld",
-  # "-Z", "linker-flavor=ld",
+  # "-C", "linker=lld",
+  # "-Z", "linker-flavor=ld.lld",
+  "-C", "linker=arm-none-eabi-ld",
+  "-Z", "linker-flavor=ld",
   "-Z", "thinlto=no",
 ]

Flash the program

$ # Launch OpenOCD on a terminal
$ openocd -f (..)

$ # Start a debug session in another terminal
$ arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/demo

Alternatively, you can use cargo run to build, flash and debug the program in a single step.

$ cargo run --example hello

```

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

