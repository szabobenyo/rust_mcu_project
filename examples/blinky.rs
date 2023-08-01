#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;
extern crate stm32l0;

use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, prelude::*, rcc::Config};

#[entry]
fn blinky() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configure LED2 as output.
    let mut led2 = gpioa.pa5.into_push_pull_output();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(rcc.clocks);

    loop {
        led2.set_high().unwrap();
        delay.delay_ms(300_u16);

        led2.set_low().unwrap();
        delay.delay_ms(300_u16);
    }
}
