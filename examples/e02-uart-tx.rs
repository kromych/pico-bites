//! Writes message to UART1
#![no_std]
#![no_main]

use core::fmt::Write;
use fugit::RateExtU32;

use defmt_rtt as _;
use panic_halt as _;

use defmt as log;
use rp2040_hal as hal;
use rp_pico as bsp;

use hal::clocks::Clock;

#[bsp::entry]
fn main() -> ! {
    log::info!("Running");

    let mut pac = bsp::pac::Peripherals::take().unwrap();
    let core = bsp::pac::CorePeripherals::take().unwrap();
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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let uart_pins = (
        pins.gpio8.into_mode::<hal::gpio::FunctionUart>(),
        pins.gpio9.into_mode::<hal::gpio::FunctionUart>(),
    );
    let mut uart = hal::uart::UartPeripheral::new(pac.UART1, uart_pins, &mut pac.RESETS)
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

    let mut count = 0u32;
    loop {
        uart.write_full_blocking(b"Counter: ");
        writeln!(uart, "{count:02}\r").unwrap();

        delay.delay_ms(100);
        count = count.wrapping_add(1);
    }
}
