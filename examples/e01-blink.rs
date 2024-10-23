//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use defmt as log;
use rp_pico as bsp;

use bsp::hal::clocks::Clock;
use embedded_hal::digital::OutputPin;

#[bsp::entry]
fn main() -> ! {
    log::info!("Running");

    let mut pac = bsp::hal::pac::Peripherals::take().unwrap();
    let core = bsp::hal::pac::CorePeripherals::take().unwrap();
    let mut watchdog = bsp::hal::watchdog::Watchdog::new(pac.WATCHDOG);
    let sio = bsp::hal::sio::Sio::new(pac.SIO);

    let clocks = bsp::hal::clocks::init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();

    loop {
        log::info!("on!");

        led_pin.set_high().unwrap();
        delay.delay_ms(500);

        log::info!("off!");

        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
