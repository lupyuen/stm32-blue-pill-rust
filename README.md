# stm32-blue-pill-rust

Rust for STM32 Blue Pill. Based on

1. https://japaric.github.io/stm32f103xx-hal/stm32f103xx_hal/

1. https://docs.rs/cortex-m-quickstart/0.2.7/cortex_m_quickstart/

```bash
sudo apt install pkg-config cmake libssl-dev zlib1g-dev
```

Nightly Rust toolchain newer than nightly-2018-04-08:
```bash
rustup default nightly

Cargo clone subcommand: 
cargo install cargo-clone

GDB: (on Ubuntu)
## Obsolete: sudo apt install gdb-arm-none-eabi
sudo apt install gdb-multiarch

OpenOCD: (on Ubuntu)
sudo apt install openocd

[Optional] ARM linker: (on Ubuntu)
sudo apt install binutils-arm-none-eabi

[Optional] Cargo add subcommand: 
cargo install cargo-edit
```

Use thumbv7m-none-eabi for ARM Cortex-M3

Install the rust-std component for your target, if you haven't done so already

```bash
rustup target add thumbv7m-none-eabi

Clone this crate

cargo clone cortex-m-quickstart && cd $_

Change the crate name, author and version

edit Cargo.toml
<<
[package]
authors = ["Jorge Aparicio <jorge@japaric.io>"]
name = "demo"
version = "0.1.0"
>>

Specify the memory layout of the target device
NOTE board support crates sometimes provide this file for you (check the crate documentation). If you are using one that does then remove both the memory.x and build.rs files.

rm memory.x build.rs

Set a default build target

cat >>.cargo/config <<'EOF'
[build]
target = "thumbv7m-none-eabi"
EOF

Depend on a HAL implementation.

cargo add https://github.com/japaric/stm32f103xx
cargo add https://github.com/japaric/stm32f103xx-hal

Edit Cargo.toml:
<<
stm32f103xx-hal = { git = "https://github.com/japaric/stm32f103xx-hal", features = ["rt"] }
>>

Write the application or start from one of the examples

rm -r src/* && cp examples/hello.rs src/main.rs

Build the application

cargo build --release
# sanity check
arm-none-eabi-readelf -A target/thumbv7m-none-eabi/release/demo

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

# Launch OpenOCD on a terminal
openocd -f (..)

# Start a debug session in another terminal
arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/demo

Alternatively, you can use cargo run to build, flash and debug the program in a single step.

cargo run --example hello

```

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
