#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, prelude::*, rcc::Config};

#[entry]
fn external_intervention() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.freeze(Config::hsi16());
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);

    let button = gpioc.pc13.into_pull_up_input();

    let mut led = gpioa.pa5.into_push_pull_output();

    let mut delay = cp.SYST.delay(rcc.clocks);

    loop {
        let wait = match button.is_high() {
            Ok(true) => 300.milliseconds(),
            Ok(false) => 10.milliseconds(),
            _ => unreachable!(),
        };
        delay.delay(wait);
        led.toggle().unwrap();
    }
}
