#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;
extern crate stm32l0;

use cortex_m_rt::entry;

#[link(name = "doubler")]
extern "C" {
    fn doubler(x: i32) -> i32;
}

#[entry]
unsafe fn cooperation() -> ! {
    let result = unsafe { doubler(5) };
    loop {
        cortex_m_semihosting::hprintln!("Doubler result: {}", result);
    }
}
