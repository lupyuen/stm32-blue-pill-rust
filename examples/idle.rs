#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![feature(proc_macro_gen)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate cortex_m_rtfm as rtfm;
extern crate panic_semihosting;
extern crate stm32f103xx;

use cortex_m::asm;
use cortex_m_rt::ExceptionFrame;
use rtfm::app;

app! {
    device: stm32f103xx,

    idle: {},
}

#[inline(always)]
fn init(_ctxt: init::Context) -> init::LateResources {
    init::LateResources {}
}

#[inline(always)]
fn idle(_ctxt: idle::Context) -> ! {
    loop {
        asm::wfi();
    }
}

exception!(HardFault, hard_fault);

#[inline(always)]
fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

#[inline(always)]
fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
