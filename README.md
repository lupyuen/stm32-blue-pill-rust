# stm32-blue-pill-rust

Rust for STM32 Blue Pill. Based on

1. https://japaric.github.io/stm32f103xx-hal/stm32f103xx_hal/

1. https://docs.rs/cortex-m-quickstart/0.2.7/cortex_m_quickstart/

1. http://nercury.github.io/rust/embedded/experiments/2018/04/29/rust-embedded-01-discovery-vl-flipping-bits.html

## Install Prerequisites

```bash
sudo apt install pkg-config cmake libssl-dev zlib1g-dev
```

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

Install the rust-std component thumbv7m-none-eabi for ARM Cortex-M3.

```bash
rustup target add thumbv7m-none-eabi
```

Build the application

```bash
cargo clean
cargo check --release --target thumbv7m-none-eabi
cargo build --release --target thumbv7m-none-eabi
# sanity check
arm-none-eabi-readelf -A target/thumbv7m-none-eabi/release/stm32-blue-pill-rust
<<
Attribute Section: aeabi
File Attributes
  Tag_CPU_name: "ARM v7"
  Tag_CPU_arch: v7
  Tag_CPU_arch_profile: Microcontroller
  Tag_THUMB_ISA_use: Thumb-2
  Tag_ABI_PCS_GOT_use: direct
  Tag_ABI_FP_denormal: Needed
  Tag_ABI_FP_exceptions: Needed
  Tag_ABI_FP_number_model: IEEE 754
  Tag_ABI_align_needed: 8-byte
  Tag_ABI_optimization_goals: Aggressive Speed
  Tag_CPU_unaligned_access: v6
  Tag_ABI_FP_16bit_format: IEEE 754
>>
```

Flash the program

```bash
# Launch OpenOCD on a terminal. Scripts are located at /usr/share/openocd/scripts
openocd -f interface/stlink.cfg -f target/stm32f1x.cfg

# Start a debug session in another terminal
arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/demo
```

Alternatively, you can use cargo run to build, flash and debug the program in a single step.

```bash
cargo run --example hello --release --target thumbv7m-none-eabi

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
<<
[package]
authors = ["Jorge Aparicio <jorge@japaric.io>"]
name = "demo"
version = "0.1.0"
>>
```

Specify the memory layout of the target device
NOTE board support crates sometimes provide this file for you (check the crate documentation). If you are using one that does then remove both the memory.x and build.rs files.

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

For Windows:

NOTE By default Cargo will use the LLD linker shipped with the Rust toolchain. If you encounter any linking error try to switch to the GNU linker by modifying the .cargo/config file as shown below:

```bash
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
