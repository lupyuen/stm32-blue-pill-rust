# stm32-blue-pill-rust

Rust for STM32 Blue Pill with Visual Studio Code. Based on

1. https://japaric.github.io/stm32f103xx-hal/stm32f103xx_hal/

1. https://docs.rs/cortex-m-quickstart/0.2.7/cortex_m_quickstart/

1. http://nercury.github.io/rust/embedded/experiments/2018/04/29/rust-embedded-01-discovery-vl-flipping-bits.html

## Install Prerequisites

```bash
sudo apt install pkg-config cmake libssl-dev zlib1g-dev
```

Install `rustup` (the Rust toolchain installer) from https://rustup.rs/

Select the default option.

Select nightly Rust toolchain newer than nightly-2018-04-08:

```bash
rustup default nightly
```

GDB: (gdb-arm-none-eabi is obsolete)

```bash
sudo apt install gdb-multiarch
```

OpenOCD:

```bash
sudo apt install openocd
```

ARM linker:

```bash
sudo apt install binutils-arm-none-eabi
```

For Windows:
Browse to `C:\Program Files (x86)\GNU Tools Arm Embedded\7 2018-q2-update\bin`.
Copy `arm-none-eabi-ar.exe` to `ar.exe`

Install the rust-std component thumbv7m-none-eabi for ARM Cortex-M3.

```bash
rustup target add thumbv7m-none-eabi
```

Download the source files

```bash
git clone https://github.com/lupyuen/stm32-blue-pill-rust.git
cd stm32-blue-pill-rust
```

Build the application

```bash
cargo clean
cargo check --release
cargo build --release
```

Sanity check for the built application

```bash
arm-none-eabi-readelf -h target/thumbv7m-none-eabi/release/stm32-blue-pill-rust
```

Should show:

```text
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0x80009d5
  Start of program headers:          52 (bytes into file)
  Start of section headers:          152956 (bytes into file)
  Flags:                             0x5000202, Version5 EABI, soft-float ABI, <unknown>
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         2
  Size of section headers:           40 (bytes)
  Number of section headers:         19
  Section header string table index: 16
```

Flash the program

```bash
# Launch OpenOCD on a terminal. Scripts are located at /usr/share/openocd/scripts
openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg
```

Should show:

```text
GNU MCU Eclipse 64-bits Open On-Chip Debugger 0.10.0+dev-00487-gaf359c18 (2018-05-12-19:30)
Licensed under GNU GPL v2
For bug reports, read http://openocd.org/doc/doxygen/bugs.html
WARNING: interface/stlink-v2.cfg is deprecated, please switch to interface/stlink.cfg
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
none separate
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : clock speed 950 kHz
Info : STLINK v2 JTAG v17 API v2 SWIM v4 VID 0x0483 PID 0x3748
Info : using stlink api v2
Info : Target voltage: 3.225397
Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Listening on port 3333 for gdb connections
```

Start a debug session in another terminal:

```bash
arm-none-eabi-gdb target/thumbv7m-none-eabi/release/stm32-blue-pill-rust
```

Alternatively, you can use cargo run to build, flash and debug the program in a single step.

```bash
cargo run --example hello --release

```

## References

Windows Setup: https://japaric.github.io/discovery/03-setup/windows.html

STM32F103C8 Website: https://www.st.com/en/microcontrollers/stm32f103c8.html

STM32F103C8 Datasheet: https://www.st.com/resource/en/datasheet/stm32f103c8.pdf

STM32F103C8 Reference Manual: https://www.st.com/content/ccc/resource/technical/document/reference_manual/59/b9/ba/7f/11/af/43/d5/CD00171190.pdf/files/CD00171190.pdf/jcr:content/translations/en.CD00171190.pdf

STM32F103C8 Flash Programming: https://www.st.com/content/ccc/resource/technical/document/programming_manual/10/98/e8/d4/2b/51/4b/f5/CD00283419.pdf/files/CD00283419.pdf/jcr:content/translations/en.CD00283419.pdf

STM32F103C8 ARM Cortex M3 Programming: https://www.st.com/content/ccc/resource/technical/document/programming_manual/5b/ca/8d/83/56/7f/40/08/CD00228163.pdf/files/CD00228163.pdf/jcr:content/translations/en.CD00228163.pdf

## How this crate was created

Install Cargo `clone` and `add` subcommands:

```bash
cargo install cargo-clone
cargo install cargo-edit
```

Clone the quickstart crate

```bash
cargo clone cortex-m-quickstart && cd $_
```

Change the crate name, author and version in Cargo.toml:

```toml
[package]
authors = ["Jorge Aparicio <jorge@japaric.io>"]
name = "demo"
version = "0.1.0"
```

Specify the memory layout of the target device.
Since board support crate for stm32f103xx provides this file, we remove both the memory.x and build.rs files.

```bash
rm memory.x build.rs
```

Set a default build target

```bash
cat >>.cargo/config <<'EOF'

[build]
target = "thumbv7m-none-eabi"
EOF
```

Depend on a HAL implementation.

```bash
cargo add https://github.com/japaric/stm32f103xx
cargo add https://github.com/japaric/stm32f103xx-hal
```

Copy the application from one of the examples

```bash
rm -r src/* && cp examples/hello.rs src/main.rs
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
