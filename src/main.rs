#![no_main]
#![no_std]

use defmt_rtt as _; // global logger

use core::cell::{Cell, RefCell};

use lora_e5_bsp::{
    hal::{
        cortex_m::{self, delay::Delay, interrupt::Mutex},
        gpio::{pins, Exti, ExtiTrg, PortA, PortB},
        info::{self, Core, Package, Uid, Uid64},
        pac::{self, interrupt},
        util::new_delay,
    },
    led,
    pb::{self, PushButton, D0},
};

#[cfg(debug_assertions)]
use panic_probe as _; // panic handler

#[cfg(not(debug_assertions))]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::free(|cs| {
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

static BLINK: Mutex<Cell<bool>> = Mutex::new(Cell::new(true));

#[cortex_m_rt::entry]
fn main() -> ! {
    // Retrieve device and core peripherals.
    let mut dp: pac::Peripherals = pac::Peripherals::take().unwrap();
    let cp: pac::CorePeripherals = pac::CorePeripherals::take().unwrap();

    // Print some infos from the board.
    defmt::debug!("CPU: {}", Core::from_cpuid());
    defmt::debug!("Flash size: {} KiB", info::flash_size_kibibyte());
    defmt::debug!("Package: {:?}", Package::from_device());
    defmt::debug!("UID64: {}", Uid64::from_device());
    defmt::debug!("UID: {}", Uid::from_device());

    // Setup the D0 button connected to pin A0.
    let gpioa: PortA = PortA::split(dp.GPIOA, &mut dp.RCC);
    let _ = cortex_m::interrupt::free(|cs| pb::D0::new(gpioa.a0, cs));

    // Setup an input pin as an EXTI interrupt source.
    <D0 as PushButton>::Pin::setup_exti_c1(&mut dp.EXTI, &mut dp.SYSCFG, ExtiTrg::Falling);
    // Unmask the interrupt in the NVIC.
    unsafe { pins::A0::unmask() };

    // Setup the D5 led connected to pin B5.
    let gpiob: PortB = PortB::split(dp.GPIOB, &mut dp.RCC);
    cortex_m::interrupt::free(|cs| LED.borrow(cs).replace(Some(led::D5::new(gpiob.b5, cs))));

    // Setup the system timer.
    let mut delay: Delay = new_delay(cp.SYST, &dp.RCC);

    defmt::info!("Starting blinky");
    loop {
        let blink = cortex_m::interrupt::free(|cs| BLINK.borrow(cs).get());
        if blink {
            cortex_m::interrupt::free(|cs| {
                let mut led = LED.borrow(cs).borrow_mut();
                led.as_mut().unwrap().set_on();
            });
            defmt::debug!("LED is on");
            delay.delay_ms(100);

            cortex_m::interrupt::free(|cs| {
                let mut led = LED.borrow(cs).borrow_mut();
                led.as_mut().unwrap().set_off();
            });
            delay.delay_ms(1000);
        } else {
            cortex_m::asm::wfi();
        }
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn EXTI0() {
    defmt::info!("Button D0 pushed");
    cortex_m::interrupt::free(|cs| {
        let val = BLINK.borrow(cs).get();
        BLINK.borrow(cs).set(!val);
    });

    pins::A0::clear_exti();
}
