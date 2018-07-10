# stm32-blue-pill-rust

Rust for STM32 Blue Pill with Visual Studio Code. 

Read the article: https://medium.com/@ly.lee/coding-the-stm32-blue-pill-with-rust-and-visual-studio-code-b21615d8a20

Based on:

1. Jorge Aparicio's Discovery book: https://japaric.github.io/discovery/

1. Jorge Aparicio's HAL for Blue Pill: https://japaric.github.io/stm32f103xx-hal/stm32f103xx_hal/

1. Jorge Aparicio's Cortex-M Quickstart: https://docs.rs/cortex-m-quickstart/0.2.7/cortex_m_quickstart/

1. Nerijus Arlauskas's Embedded Rust blog: http://nercury.github.io/rust/embedded/experiments/2018/04/29/rust-embedded-01-discovery-vl-flipping-bits.html

-----
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

-----
## Installation and Usage

### Install Prerequisites

- For Ubuntu only: Install required packages  (`arm-none-eabi-gdb` is obsolete)

  ```bash
  sudo apt install pkg-config cmake libssl-dev zlib1g-dev gdb-multiarch curl git
  
  sudo ln -s /usr/bin/gdb-multiarch /usr/bin/arm-none-eabi-gdb
  ```

### Install ARM Cross-Compiler and Linker

- For Windows:

  1. Install ARM Cross-Compiler and Linker from the ARM Developer Website: <br>
    https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads

  1. Scroll down the page till you find <br>
    `Windows 32-bit File: gcc-arm-none-eabi-…-win32.exe` <br>
    Click `Download` <br>

  1. Select the _"Add path to environment variable"_ option at the last install step

  1. In Windows Explorer, browse to <br>
    `C:\Program Files (x86)\GNU Tools Arm Embedded\7 2018-q2-update\bin` <br>
    (The `7 2018-q2-update` part may be different for your installation)

  1. Copy the file `arm-none-eabi-ar.exe` to `ar.exe` <br>
    (This `ar.exe` workaround is temporary until we find a fix for the Windows Rust build)

- For Ubuntu:

  ```bash
  sudo apt install binutils-arm-none-eabi gcc-arm-none-eabi
  ```

### Check ARM Cross-Compiler Installation

1. Open a __new__  Windows or Ubuntu command prompt (not Windows Bash) and enter

    ```bash
    arm-none-eabi-gcc -v
    ```

1. You should see something like `version 5.4.1 20160919 (release)`

1. If you see no errors, close the command prompt.

1. If you see an error, update your PATH environment variable so that it  includes the folder for the ARM ".exe" files.

### Install OpenOCD For Debugging Blue Pill

- For Windows:

  1. Download OpenOCD (for debugging the Blue Pill) from the unofficial OpenOCD release website: <br>
    https://github.com/gnu-mcu-eclipse/openocd/releases <br>
    Look for `gnu-mcu-eclipse-openocd-…-win64.zip`

  1. Unzip the OpenOCD download and copy the OpenOCD files into `c:\openocd` such that `opencd.exe` is located in the folder `c:\openocd\bin`

- For Ubuntu:

  ```bash
  sudo apt install openocd
  ```

### For Windows only: Install ST-Link USB Driver

1. For Windows only: Download the ST-Link USB driver from the ST-Link Driver Website (email registration required): <br>
  http://www.st.com/en/embedded-software/stsw-link009.html

1. Scroll down and click the `Get Software` button

1. Unzip the ST-Link download. Double-click the `dpinst_amd64.exe` installer.

### For Windows only: Install Build Tools

1. For Windows only: Install `Build Tools for Visual Studio 2017` (needed by `rustup`) from <br>
    https://aka.ms/buildtools

1. Under "Workloads", select `Visual C++ Build Tools`. <br>
    _Warning: The download is 1.1 GB and you need 4.8 GB of free disk space._

### Install `rustup`

1. Install `rustup` (the Rust toolchain installer) from <br>
    https://rustup.rs/

1. Select the default option when prompted. <br>
    For Ubuntu only: Log out and log in again to update the PATH

1. Switch to the nightly Rust toolchain (instead of stable or beta). Open a __new__  Windows or Ubuntu command prompt (not Windows Bash) and enter:

    ```bash
    rustup default nightly
    ```

1. Install the `rust-std` component `thumbv7m-none-eabi` to cross-compile for ARM Cortex-M3 (the processor used in the Blue Pill):

    ```bash
    rustup target add thumbv7m-none-eabi
    ```

### Download `stm32-blue-pill-rust` Source Files

- Download the source files from GitHub: <br>
  (You may install git from https://gitforwindows.org)

  ```bash
  git clone https://github.com/lupyuen/stm32-blue-pill-rust.git
  cd stm32-blue-pill-rust
  ```

### Install Visual Studio Code

1. Install Visual Studio Code from <br>
  https://code.visualstudio.com/download

1. Launch Visual Studio Code and install the following extensions (just click the links below followed by the `Install` button and `Open Visual Studio Code`):

    - Better TOML (bungcip) <br>
      https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml

    - C/C++ (Microsoft) <br>
      https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools

    - Native Debug (WebFreak) <br>
      https://marketplace.visualstudio.com/items?itemName=webfreak.debug

    - Rust (kalitaalexey) <br>
      https://marketplace.visualstudio.com/items?itemName=kalitaalexey.vscode-rust

    - Rust (rls) (rust-lang) <br>
      https://marketplace.visualstudio.com/items?itemName=rust-lang.rust

1. In Visual Studio Code, click `Install` when prompted to install the above extensions

1. Restart Visual Studio Code

1. Click `File → Open Workspace`

1. Browse to the `stm32-blue-pill-rust` folder and select `workspace.code-workspace`

1. In the `Explorer → Workspace` pane at left, browse to the source folder `src` and select the Rust source file `main.rs`

1. When prompted, install the Rust Language Service (RLS), which provides Autocomplete and "Go To Definition" features for Rust.

### Compiling the Rust program in Visual Studio Code

1. In Visual Studio Code, click `Tasks → Run` Build Task.

1. Wait a while for the Rust program to be compiled.

1. Check the log in the Terminal window at the bottom of the Visual Studio Code screen.

1. When you see `Finished released [optimized + debuginfo] target(s)`, that means the Rust program has been compiled successfully.

1. We'll proceed to the next step to run the program.

1. But if you see an error, you'll have to fix the error and recompile the program.
  Just mouse over the filename and line number in the log, and press Ctrl-click to jump to the offending line of code.

### Running the Rust program in Visual Studio Code

1. Click `Tasks → Run Task`

1. Select `Connect To STM32 Blue Pill`

1. Check the messages from OpenOCD in the Terminal window at the bottom of Visual Studio Code.

1. When you see `Listening on port 3333 for gdb connections`, our program is ready to be started on the Blue Pill.

1. Click `Debug → Start Debugging`

1. Note: There is a bug in the debugger for Ubuntu: gdb stops with an error. To be fixed.
  Meanwhile you can use the command-line debugger in Ubuntu.

### Building from the Command Line

1. Build the application:

    ```bash
    cargo clean
    cargo check --release
    cargo build --release
    ```

1. You should see something like:

    ```text
    Finished release [optimized + debuginfo] target(s) in 1.43s
    ```

1. Sanity check for the built application

    ```bash
    arm-none-eabi-readelf -h target/thumbv7m-none-eabi/release/stm32-blue-pill-rust
    ```

1. You should see something like:

    ```text
    ELF Header:
      Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
      Class:                             ELF32
      Data:                              2's complement, little endian
      Version:                           1 (current)
      OS/ABI:                            UNIX 1. System V
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

### Debugging from the Command Line

1. Launch OpenOCD on a terminal. Scripts are located at `/usr/share/openocd/scripts`

    - For Windows:

      ```cmd
      c:\openocd\bin\openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg
      ```

    - For Ubuntu:

      ```bash
      openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg
      ```

1. You should see something like:

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

1. Start a debug session in another command window:

    ```bash
    arm-none-eabi-gdb -x loader.gdb target/thumbv7m-none-eabi/release/stm32-blue-pill-rust
    ```

1. Common GDB commands:

    - `step:` Execute the current source line, step into functions if present. Same as the step into  command in Visual Studio Code.

    - `next:` Execute the current source line, don't step into functions. Same as the step over command in Visual Studio Code.

    - `where:` Show stack trace.

    - `where full:` Show stack trace with local variables.

    More commands: https://darkdust.net/files/GDB%20Cheat%20Sheet.pdf

-----
## Visual Studio Code Configuration

Customisation of the Visual Studio Code UI was done through the following files:

### Task configuration - `.vscode/tasks.json`

Defines the following tasks:

1. Connect: Launches OpenOCD. Uses different commands for Ubuntu and Windows (`opencd` vs `c:\opencd\bin\opencd`).

1. Build: Executes `cargo build --release`. Configured as the default build task.

1. Remove: Executes `cargo clean`

1. Check: Executes `cargo check --release`

### Debugger configuration - `.vscode/launch.json`

- Launches the following command when the debugger is started:

  ```bash
  arm-none-eabi-gdb -x loader.gdb target/thumbv7m-none-eabi/release/stm32-blue-pill-rust
  ```

- This command causes `gdb` to execute the `loader.gdb` script at the start of debugging.

- The mandatory parameter `target/thumbv7m-none-eabi/release/stm32-blue-pill-rust` is redundant. The target is specified again in `loader.gdb.`

### Debugger script - `loader.gdb`

1. This is the GDB script for loading and running programs in STM32 Blue Pill.

1. Called when debugging begins. Defined in `.vscode/launch.json`

1. This file used to be `.gdbinit,` which could not be autoloaded due to autoloading security in GDB.

1. Set architecture to ARM 32-bit. Needed for `gdb-multiarch` on Ubuntu.

1. Send GDB commands to OpenOCD, which listens on port 3333.  Extend the timeout.

1. Disable all messages.

1. Enable ARM semihosting to show debug console output in OpenOCD console.

1. Reset the device.

1. Specify `target/thumbv7m-none-eabi/release/stm32-blue-pill-rust` as the target program to be debugged.  Must be specified here (not the command line) because the VSCode debugger will fail without it.

1. Load the program into device memory.

1. Set breakpoint at the `main()` function.

1. Run the program and stop at the `main()` function.

1. Remove the breakpoint at the `main()` function.

1. Step into the first line of the `main()` function. Else gdb will complain about `entry macros` file missing.

1. TODO: Write program to flash memory so that it becomes permanent.

-----
## References

Windows Setup: https://japaric.github.io/discovery/03-setup/windows.html

STM32F103C8 Website: https://www.st.com/en/microcontrollers/stm32f103c8.html

STM32F103C8 Datasheet: https://www.st.com/resource/en/datasheet/stm32f103c8.pdf

STM32F103C8 Reference Manual: https://www.st.com/content/ccc/resource/technical/document/reference_manual/59/b9/ba/7f/11/af/43/d5/CD00171190.pdf/files/CD00171190.pdf/jcr:content/translations/en.CD00171190.pdf

STM32F103C8 Flash Programming: https://www.st.com/content/ccc/resource/technical/document/programming_manual/10/98/e8/d4/2b/51/4b/f5/CD00283419.pdf/files/CD00283419.pdf/jcr:content/translations/en.CD00283419.pdf

STM32F103C8 ARM Cortex M3 Programming: https://www.st.com/content/ccc/resource/technical/document/programming_manual/5b/ca/8d/83/56/7f/40/08/CD00228163.pdf/files/CD00228163.pdf/jcr:content/translations/en.CD00228163.pdf

-----
## How This Rust Crate Was Created

1. Install Cargo `clone` and `add` subcommands:

    ```bash
    cargo install cargo-clone
    cargo install cargo-edit
    ```

1. Clone the quickstart crate

    ```bash
    cargo clone cortex-m-quickstart && cd $_
    ```

1. Change the crate name, author and version in Cargo.toml:

    ```toml
    [package]
    authors = ["Jorge Aparicio <jorge@japaric.io>"]
    name = "demo"
    version = "0.1.0"
    ```

1. Specify the memory layout of the target device.
Since board support crate for stm32f103xx provides this file, we remove both the memory.x and build.rs files.

    ```bash
    rm memory.x build.rs
    ```

1. Set a default build target

    ```bash
    cat >>.cargo/config <<'EOF'

    [build]
    target = "thumbv7m-none-eabi"
    EOF
    ```

1. Depend on a HAL implementation.

    ```bash
    cargo add https://github.com/japaric/stm32f103xx
    cargo add https://github.com/japaric/stm32f103xx-hal
    ```

1. Copy the `delay` sample application from https://github.com/japaric/stm32f103xx-hal into `src\main.rs`

    ```bash
    rm -r src/* && cp ../stm32f103xx_hal/examples/delay.rs src/main.rs
    ```

-----
## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

-----
## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
