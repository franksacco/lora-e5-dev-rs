#![no_main]
#![no_std]

use defmt_rtt as _; // global logger

use core::cell::RefCell;

use cortex_m::{
    delay::Delay,
    interrupt::{self, Mutex},
};
use lora_e5_bsp::{
    hal::{
        gpio::PortB,
        info::{self, Core, Package, Uid, Uid64},
        pac,
        util::new_delay,
    },
    led,
};

#[cfg(debug_assertions)]
use panic_probe as _; // panic handler

#[cfg(not(debug_assertions))]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    interrupt::free(|cs| {
        let mut led = LED.borrow(cs).borrow_mut();
        if let Some(ref mut led) = led.as_mut() {
            loop {
                for _ in 0..100_000 {
                    cortex_m::asm::nop();
                }
                led.set_on();
                for _ in 0..100_000 {
                    cortex_m::asm::nop();
                }
                led.set_off();
            }
        }
    });

    loop {
        cortex_m::asm::nop();
    }
}

// Global static access to the board led, used to report errors.
static LED: Mutex<RefCell<Option<led::D5>>> = Mutex::new(RefCell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut dp: pac::Peripherals = pac::Peripherals::take().unwrap();
    let cp: pac::CorePeripherals = pac::CorePeripherals::take().unwrap();

    // Print some infos from the board.
    defmt::println!("CPU: {}", Core::from_cpuid());
    defmt::println!("Flash size: {} KiB", info::flash_size_kibibyte());
    defmt::println!("Package: {:?}", Package::from_device());
    defmt::println!("UID64: {}", Uid64::from_device());
    defmt::println!("UID: {}", Uid::from_device());

    let gpiob: PortB = PortB::split(dp.GPIOB, &mut dp.RCC);
    interrupt::free(|cs| LED.borrow(cs).replace(Some(led::D5::new(gpiob.b5, cs))));

    let mut delay: Delay = new_delay(cp.SYST, &dp.RCC);

    defmt::info!("Starting blinky");
    loop {
        interrupt::free(|cs| {
            let mut led = LED.borrow(cs).borrow_mut();
            led.as_mut().unwrap().set_on();
        });
        defmt::debug!("LED is on");
        delay.delay_ms(100);

        interrupt::free(|cs| {
            let mut led = LED.borrow(cs).borrow_mut();
            led.as_mut().unwrap().set_off();
        });
        delay.delay_ms(2000);
    }
}
