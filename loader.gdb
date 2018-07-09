# GDB script for loading and running programs in STM32 Blue Pill.
# This file used to be .gdbinit, which could not be autoloaded due to autoloading security in GDB.

# Set architecture to ARM 32-bit. Needed for gdb-multiarch on Ubuntu.
set architecture arm

# Send GDB commands to OpenOCD, which listens on port 3333.  Extend the timeout.
set remotetimeout 100000
target remote :3333

# Print demangled symbols by default.
set print asm-demangle on

# Enable ARM semihosting to show debug console output in OpenOCD console.
monitor arm semihosting enable

# Reset the device.
monitor reset init
monitor sleep 20
monitor halt
monitor sleep 20

# Load the program into device memory.
file target/thumbv7m-none-eabi/release/stm32-blue-pill-rust
load

# Set breakpoint at the main() function.
break stm32_blue_pill_rust::main

# Run the program and stop at the main() function.
continue

# Remove the breakpoint at the main() function.
#clear stm32_blue_pill_rust::main

####
#monitor sleep 300

##########################################################################
# Optional Commands

# Send captured ITM to the file itm.fifo
# (the microcontroller SWO pin must be connected to the programmer SWO pin)
# 8000000 must match the core clock frequency:
# monitor tpiu config internal itm.fifo uart off 8000000

# OR: make the microcontroller SWO pin output compatible with UART (8N1)
# 2000000 is the frequency of the SWO pin:
# monitor tpiu config external uart off 8000000 2000000

# Enable ITM port 0:
# monitor itm port 0 on
