# GDB script for flashing and running programs in STM32 Blue Pill.
# This file used to be .gdbinit, which could not be autoloaded due to autoloading security in GDB.

# Set architecture to ARM 32-bit. Needed for gdb-multiarch.
set architecture arm

# Send GDB commands to OpenOCD, which listens on port 3333.
set remotetimeout 100000
target remote :3333

# Disable all messages
set verbose off
set complaints 0
set confirm off
set exec-done-display off
show exec-done-display
set trace-commands off
# set debug aix-thread off
# set debug dwarf2-die 0
set debug displaced off 
set debug expression 0
set debug frame 0
set debug infrun 0
set debug observer 0
set debug overload 0
# Lup Yuen: Removed due to error.
# set debugvarobj 0
set pagination off
set print address off
set print symbol-filename off
set print symbol off
set print pretty off
set print object off
#set debug notification off
set debug parser off
set debug remote 0

# Print demangled symbols by default.
set print asm-demangle on

# Reset the device.
monitor reset init
monitor sleep 20
monitor halt
monitor sleep 20

# Enable ARM semihosting.
monitor arm semihosting enable

# Load the program into flash memory.
file target/thumbv7m-none-eabi/release/stm32-blue-pill-rust

# Set breakpoint at the main() function.
break stm32_blue_pill_rust::main

# Run the program and stop at the main function.
continue

# Remove the breakpoint at the main() function.
## clear stm32_blue_pill_rust::main

# Send captured ITM to the file itm.fifo
# (the microcontroller SWO pin must be connected to the programmer SWO pin)
# 8000000 must match the core clock frequency
# monitor tpiu config internal itm.fifo uart off 8000000

# OR: make the microcontroller SWO pin output compatible with UART (8N1)
# 2000000 is the frequency of the SWO pin
# monitor tpiu config external uart off 8000000 2000000

# Enable ITM port 0
# monitor itm port 0 on
