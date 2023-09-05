//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
//! The implementation uses software tasks in RTIC and the busy wait rather than the hardware timer interrupt.
#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = rp_pico::hal::pac, peripherals = true, dispatchers = [SW0_IRQ])]
mod app {
    use defmt as log;

    use rp2040_hal as hal;
    use rp_pico as bsp;

    use embedded_hal::digital::v2::ToggleableOutputPin;
    use hal::gpio::FunctionSio;
    use hal::gpio::PullDown;
    use hal::gpio::SioOutput;
    use hal::Clock;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led_pin: hal::gpio::Pin<hal::gpio::bank0::Gpio25, FunctionSio<SioOutput>, PullDown>,
        delay: cortex_m::delay::Delay,
        led_on: bool,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        log::info!("RTIC app init");

        let mut pac = cx.device;
        let sio = bsp::hal::sio::Sio::new(pac.SIO);

        let mut watchdog = bsp::hal::watchdog::Watchdog::new(pac.WATCHDOG);
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

        let pins = bsp::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );
        let led_pin = pins
            .led
            .into_push_pull_output_in_state(hal::gpio::PinState::Low);

        let core = cx.core;
        let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

        blink::spawn().unwrap();

        (
            Shared {},
            Local {
                led_pin,
                delay,
                led_on: false,
            },
            init::Monotonics(),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            log::info!("RTIC idle");
            cortex_m::asm::wfe();
        }
    }

    #[task(local = [led_pin, delay, led_on])]
    fn blink(cx: blink::Context) {
        let blink::LocalResources {
            delay,
            led_pin,
            led_on,
        } = cx.local;

        led_pin.toggle().ok();
        *led_on = !*led_on;

        if *led_on {
            log::info!("tick!");
        } else {
            log::info!("tock!");
        }

        delay.delay_ms(500);
        blink::spawn().unwrap();
    }
}
