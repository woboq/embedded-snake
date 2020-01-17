#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

use stm32f4::stm32f412;
use stm32f4xx_hal::{stm32, prelude::*};

#[entry]
fn main() -> ! {

    let dp = stm32::Peripherals::take().unwrap();

    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpioa = dp.GPIOA.split();

    let mut led_wifi = gpiob.pb2.into_push_pull_output();
    let mut led_user = gpioc.pc13.into_push_pull_output();
    let mut led_azure = gpioa.pa15.into_push_pull_output();

    let mut led_r = gpiob.pb4.into_push_pull_output();
    let mut led_b = gpioc.pc7.into_push_pull_output();
    let mut led_g = gpiob.pb3.into_push_pull_output();

    let btn_a = gpioa.pa4.into_pull_up_input();
    let btn_b = gpioa.pa10.into_pull_up_input();

    loop {

        led_b.set_high();
        cortex_m::asm::delay(2000000);
        led_b.set_low();
        cortex_m::asm::delay(2000000);
    }
}
