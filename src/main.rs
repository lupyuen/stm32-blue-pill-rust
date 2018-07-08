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

use hal::delay::Delay;
use hal::prelude::*;
use hal::stm32f103xx;
use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    //  Get peripherals for the STM32 Blue Pill microcontroller.
    let dp = stm32f103xx::Peripherals::take().unwrap();
    //  Get peripherals common to ARM Cortex M processors.
    let cp = cortex_m::Peripherals::take().unwrap();

    //  Get the flash memory parts.
    let mut flash = dp.FLASH.constrain();
    //  Get the STM32 Reset and Clock Control (RCC).
    let mut rcc = dp.RCC.constrain();

    //  Get the clocks from the RCC and freeze the Access Control Register.
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    //  Get GPIO Port C. Must enable the Advanced Peripheral Bus 2 (APB2) clock for Port C so that GPIO will work.
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    //  Use Pin 13 of the Blue Pill for GPIO Port C. Select Output Push/Pull mode for the pin, which is connected to our LED.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    //  Create a delay timer from the RCC clocks.
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
