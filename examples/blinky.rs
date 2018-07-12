//! Blink the LED. From http://blog.japaric.io/rtfm-v2/
#![feature(proc_macro)]
#![feature(proc_macro_gen)]
#![no_std]

//// extern crate blue_pill;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;

use blue_pill::led::{self, Green};
use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Threshold};

mod blue_pill {
}

app! {
    device: blue_pill::stm32f103xx,

    resources: {
        static ON: bool = false;
    },

    free_interrupts: [EXTI0],

    tasks: {
        SYS_TICK: {
            path: toggle,
            resources: [ON],
        },
    },
}

fn init(p: init::Peripherals, _r: init::Resources) {
    led::init(p.GPIOC, p.RCC);

    // Configure the system timer to generate periodic events at 1 Hz rate
    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(8_000_000); // Period = 1s
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();
}

fn idle() -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS

// Toggle the state of the LED
fn toggle(_t: &mut Threshold, r: SYS_TICK::Resources) {
    **r.ON = !**r.ON;

    if **r.ON {
        Green.on();
    } else {
        Green.off();
    }
}
