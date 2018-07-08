//! "Blinky" using delays instead of a timer

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f103xx_hal as hal;

use rt::ExceptionFrame;
use hal::delay::Delay;
use hal::prelude::*;
use hal::stm32f103xx::Peripherals;

entry!(main);

fn main() -> ! {
    //  Get peripherals for the STM32 Blue Pill microcontroller.
    let bluepill = Peripherals::take().unwrap();

    //  Get the clocks from the STM32 Reset and Clock Control (RCC) and freeze the Flash Access Control Register (ACR).
    let mut rcc = bluepill.RCC.constrain();
    let mut flash = bluepill.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    //  Get GPIO Port C, which also enables the Advanced Peripheral Bus 2 (APB2) clock for Port C.
    let mut gpioc = bluepill.GPIOC.split(&mut rcc.apb2);

    //  Use Pin 13 of the Blue Pill for GPIO Port C. Select Output Push/Pull mode, which is connected to our LED.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    //  Create a delay timer from the RCC clocks.
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, clocks);

    //  Loop forever.
    loop {
        //  Output 3.3V on the LED Pin.
        led.set_high();

        //  Wait 1,000 millisec (1 sec).
        delay.delay_ms(1_000_u16);

        //  Output 0V on the LED Pin.
        led.set_low();
        
        //  Wait 1,000 millisec (1 sec).
        delay.delay_ms(1_000_u16);
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
