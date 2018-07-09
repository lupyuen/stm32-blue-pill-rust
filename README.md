# stm32-blue-pill-rust

Rust for STM32 Blue Pill with Visual Studio Code. Based on

1. Jorge Aparicio's Discovery book: https://japaric.github.io/discovery/

1. Jorge Aparicio's HAL for Blue Pill: https://japaric.github.io/stm32f103xx-hal/stm32f103xx_hal/

1. Jorge Aparicio's Cortex-M Quickstart: https://docs.rs/cortex-m-quickstart/0.2.7/cortex_m_quickstart/

1. Nerijus Arlauskas's Embedded Rust blog: http://nercury.github.io/rust/embedded/experiments/2018/04/29/rust-embedded-01-discovery-vl-flipping-bits.html

## Connecting the STM32 Blue Pill to ST-Link V2 USB Debugger

<table>
    <thead>
        <td colspan="2">
            <b> STM32 Blue Pill </b>
        </td>
        <td colspan="2">
            <b> ST-Link V2 USB Debugger </b>
        </td>
    </thead>
    <tbody>
        <tr>
            <td> V3 </td><td> [Red] </td>
            <td> 3.3V </td><td> (Pin 8) </td>
        </tr>
        <tr>
            <td> IO </td><td> [Orange] </td>
            <td> SWDIO </td><td> (Pin 4) </td>
        </tr>
        <tr>
            <td> CLK </td><td> [Brown] </td>
            <td> SWDCLK </td><td> (Pin 2) </td>
        </tr>
        <tr>
            <td> GND </td><td> [Black] </td>
            <td> GND </td><td> (Pin 6) </td>
        </tr>
    </tbody>
</table>

## Installation and Usage

### Install prerequisites

- For Ubuntu only: Install required packages  (`arm-none-eabi-gdb` is obsolete)

  ```bash
  sudo apt install pkg-config cmake libssl-dev zlib1g-dev gdb-multiarch
  
  sudo ln -s /usr/bin/gdb-multiarch /usr/bin/arm-none-eabi-gdb
  ```

### Install ARM cross-compiler and linker

- For Windows:

  - Install from https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads

  - Select the _"Add path to environment variable"_ option at the last step.

  - Browse to the folder `C:\Program Files (x86)\GNU Tools Arm Embedded\7 2018-q2-update\bin`

  - The `"7 2018-q2-update"` part may be different for your installation

  - Copy `arm-none-eabi-ar.exe` to `ar.exe`

  - This `ar.exe` fix is temporary until we find a fix for the Windows build

- For Ubuntu:

  ```bash
  sudo apt install binutils-arm-none-eabi
  ```

### Check ARM cross-compiler installation

- Open a command prompt and run

  ```bash
  arm-none-eabi-gcc -v
  ```

- You should see something like:

  ```text
  gcc version 5.4.1 20160919 (release)
  ```

### Install OpenOCD for debugging the Blue Pill

- For Windows:

  - Download the unofficial release from https://github.com/gnu-mcu-eclipse/openocd/releases

  - Unzip and copy the files into `c:\openocd` such that `opencd.exe` is located in the folder `c:\openocd\bin\`

- For Ubuntu:

  ```bash
  sudo apt install openocd
  ```

### Install ST-Link USB driver

- For Windows only: Install the ST-Link USB driver from

  http://www.st.com/en/embedded-software/stsw-link009.html

  If you're using 64-bit Windows, select the 64-bit version of the driver.

### Install `rustup`

- Install `rustup` (the Rust toolchain installer) from https://rustup.rs/

- Select the default option when prompted.

- Switch to the nightly Rust toolchain (instead of stable or beta):

  ```bash
  rustup default nightly
  ```

- Install the `rust-std` component `thumbv7m-none-eabi` to cross-compile for ARM Cortex-M3 (the processor used in the Blue Pill):

  ```bash
  rustup target add thumbv7m-none-eabi
  ```

### Download `stm32-blue-pill-rust` files

- Download the source files from GitHub (install `git` if you haven't):

  ```bash
  git clone https://github.com/lupyuen/stm32-blue-pill-rust.git
  cd stm32-blue-pill-rust
  ```

### Install Visual Studio Code

- Install Visual Studio Code from ...

- Launch Visual Studio Code

- Install the following Visual Studio Code extensions by clicking these links: ...

- In Visual Studio Code, click `File -> Open Workspace`

- Browse to the `stm32-blue-pill-rust` folder and select `workspace.code-workspace`

### Building and Debugging from Visual Studio Code

- Building: ...

- Start OpenOCD: ...

- Debugging: ...

### Building and Debugging from the Command Line

- Build the application:

  ```bash
  cargo clean
  cargo check --release
  cargo build --release
  ```

- You should see something like:

  ```text
  Finished release [optimized + debuginfo] target(s) in 1.43s
  ```

  Sanity check for the built application

  ```bash
  arm-none-eabi-readelf -h target/thumbv7m-none-eabi/release/stm32-blue-pill-rust
  ```

- You should see something like:

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
    Entry point address:               0x8000cfb
    Start of program headers:          52 (bytes into file)
    Start of section headers:          258948 (bytes into file)
    Flags:                             0x5000200, Version5 EABI, soft-float ABI
    Size of this header:               52 (bytes)
    Size of program headers:           32 (bytes)
    Number of program headers:         3
    Size of section headers:           40 (bytes)
    Number of section headers:         21
    Section header string table index: 20
  ```

- Launch OpenOCD on a terminal. Scripts are located at /usr/share/openocd/scripts

  - For Windows:

    ```cmd
    c:\openocd\bin\openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg
    ```

  - For Ubuntu:

    ```bash
    openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg
    ```

- You should see something like:

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

- Run the program on the device: Start a debug session in another command window:

  ```bash
  arm-none-eabi-gdb -x loader.gdb target/thumbv7m-none-eabi/release/stm32-blue-pill-rust
  ```

## Task configuration: `.vscode/tasks.json`

## Debugger configuration: `.vscode/launch.json`

## Debugger script: `loader.gdb`

## References

Windows Setup: https://japaric.github.io/discovery/03-setup/windows.html

STM32F103C8 Website: https://www.st.com/en/microcontrollers/stm32f103c8.html

STM32F103C8 Datasheet: https://www.st.com/resource/en/datasheet/stm32f103c8.pdf

STM32F103C8 Reference Manual: https://www.st.com/content/ccc/resource/technical/document/reference_manual/59/b9/ba/7f/11/af/43/d5/CD00171190.pdf/files/CD00171190.pdf/jcr:content/translations/en.CD00171190.pdf

STM32F103C8 Flash Programming: https://www.st.com/content/ccc/resource/technical/document/programming_manual/10/98/e8/d4/2b/51/4b/f5/CD00283419.pdf/files/CD00283419.pdf/jcr:content/translations/en.CD00283419.pdf

STM32F103C8 ARM Cortex M3 Programming: https://www.st.com/content/ccc/resource/technical/document/programming_manual/5b/ca/8d/83/56/7f/40/08/CD00228163.pdf/files/CD00228163.pdf/jcr:content/translations/en.CD00228163.pdf

## How this Rust crate was created

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

Copy the application from the `delay` example from https://github.com/japaric/stm32f103xx-hal

```bash
rm -r src/* && cp ../stm32f103xx_hal/examples/delay.rs src/main.rs
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
