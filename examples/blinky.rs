//! Blink the LED. From http://blog.japaric.io/rtfm-v2/
#![feature(proc_macro)]
#![feature(proc_macro_gen)]
#![no_std]

//// extern crate blue_pill;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f103xx_hal as hal;  //  Hardware Abstraction Layer (HAL) for STM32 Blue Pill.

use blue_pill::led::{self, LD13};
use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Threshold};

mod blue_pill {
    mod led {
        //! On-board user LEDs

        use core::ops;

        use hal::prelude::*;

        //// use hal::gpio::gpioe::{self, PE10, PE11, PE12, PE13, PE14, PE15, PE8, PE9, PEx};
        use hal::gpio::gpioc::{self, PC13};
        use hal::gpio::{Output, PushPull};

        //// pub type LD10 = PE13<Output<PushPull>>;
        pub type LD13 = PC13<Output<PushPull>>;

        /// Array of all the user LEDs on the board
        pub struct Leds {
            leds: [Led; 1],
        }

        impl Leds {
            /// Initializes all the user LEDs
            pub fn new(mut gpioc: gpioc::Parts) -> Self {
                let ld13 = gpioc
                    .pc13
                    .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

                Leds {
                    leds: [
                        ld13.into(),
                    ],
                }
            }
        }

        impl ops::Deref for Leds {
            type Target = [Led];

            fn deref(&self) -> &[Led] {
                &self.leds
            }
        }

        impl ops::DerefMut for Leds {
            fn deref_mut(&mut self) -> &mut [Led] {
                &mut self.leds
            }
        }

        impl ops::Index<usize> for Leds {
            type Output = Led;

            fn index(&self, i: usize) -> &Led {
                &self.leds[i]
            }
        }

        impl ops::Index<Direction> for Leds {
            type Output = Led;

            fn index(&self, d: Direction) -> &Led {
                &self.leds[d as usize]
            }
        }

        impl ops::IndexMut<usize> for Leds {
            fn index_mut(&mut self, i: usize) -> &mut Led {
                &mut self.leds[i]
            }
        }

        impl ops::IndexMut<Direction> for Leds {
            fn index_mut(&mut self, d: Direction) -> &mut Led {
                &mut self.leds[d as usize]
            }
        }

        /// One of the on-board user LEDs
        pub struct Led {
            pex: PEx<Output<PushPull>>,
        }

        macro_rules! ctor {
            ($($ldx:ident),+) => {
                $(
                    impl Into<Led> for $ldx {
                        fn into(self) -> Led {
                            Led {
                                pex: self.downgrade(),
                            }
                        }
                    }
                )+
            }
        }

        ctor!(LD13);

        impl Led {
            /// Turns the LED off
            pub fn off(&mut self) {
                self.pex.set_low()
            }

            /// Turns the LED on
            pub fn on(&mut self) {
                self.pex.set_high()
            }
        }

    }
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
        LD13.on();
    } else {
        LD13.off();
    }
}
