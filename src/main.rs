#![no_std]
#![no_main]

mod log;

use cortex_m::{delay::Delay, interrupt};
use cortex_m_rt::entry;

use lora_e5_bsp::{
    hal::{gpio::PortB, pac, util::new_delay},
    led,
};

// Dev profile: easier to debug panics when in debug
#[cfg(debug_assertions)]
use panic_semihosting as _;

// Release profile: minimize the binary size of the application
#[cfg(not(debug_assertions))]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    let mut dp: pac::Peripherals = pac::Peripherals::take().unwrap();
    let cp: pac::CorePeripherals = pac::CorePeripherals::take().unwrap();

    let gpiob: PortB = PortB::split(dp.GPIOB, &mut dp.RCC);
    let mut led = interrupt::free(|cs| led::D5::new(gpiob.b5, cs));
    led.set_off();

    let mut delay: Delay = new_delay(cp.SYST, &dp.RCC);

    log::log!("Starting blinky");
    loop {
        delay.delay_ms(1000);
        led.set_on();
        log::log!("LED is on");

        delay.delay_ms(100);
        led.set_off();
    }
}
