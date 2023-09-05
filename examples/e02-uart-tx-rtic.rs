#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = rp_pico::hal::pac, peripherals = true, dispatchers = [SW0_IRQ])]
mod app {
    use core::fmt::Write;

    use defmt as log;

    use rp2040_hal as hal;
    use rp_pico as bsp;

    use hal::gpio::Pin;
    use hal::uart::UartPeripheral;
    use hal::Clock;

    use fugit::RateExtU32;
    use hal::gpio::FunctionUart;
    use hal::gpio::PullDown;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        uart: UartPeripheral<
            hal::uart::Enabled,
            bsp::pac::UART1,
            (
                Pin<hal::gpio::bank0::Gpio8, FunctionUart, PullDown>,
                Pin<hal::gpio::bank0::Gpio9, FunctionUart, PullDown>,
            ),
        >,
        delay: cortex_m::delay::Delay,
        count: u32,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        log::info!("RTIC app init");

        let mut pac = cx.device;
        let core = cx.core;

        let sio = bsp::hal::sio::Sio::new(pac.SIO);
        let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
        let clocks = hal::clocks::init_clocks_and_plls(
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

        let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
        let pins = hal::gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        let uart_pins = (
            pins.gpio8.into_function::<hal::gpio::FunctionUart>(),
            pins.gpio9.into_function::<hal::gpio::FunctionUart>(),
        );
        let uart = hal::uart::UartPeripheral::new(pac.UART1, uart_pins, &mut pac.RESETS)
            .enable(
                hal::uart::UartConfig::new(
                    115200.Hz(),
                    hal::uart::DataBits::Eight,
                    None,
                    hal::uart::StopBits::One,
                ),
                clocks.peripheral_clock.freq(),
            )
            .unwrap();

        send_count::spawn().unwrap();

        (
            Shared {},
            Local {
                uart,
                delay,
                count: 0,
            },
            init::Monotonics {},
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            log::info!("RTIC idle");
            cortex_m::asm::wfe();
        }
    }

    #[task(local = [uart, delay, count])]
    fn send_count(cx: send_count::Context) {
        let send_count::LocalResources { uart, delay, count } = cx.local;

        uart.write_full_blocking(b"Counter: ");
        writeln!(uart, "{count:02}\r").unwrap();

        delay.delay_ms(100);

        *count = count.wrapping_add(1);
        send_count::spawn().unwrap();
    }
}
