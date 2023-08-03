#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;
extern crate stm32l0;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {
        panic!("");
    }
}
