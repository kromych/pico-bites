//! Shows off working with the ST7789 TFT display.
//!
//! This will print "Running" in the embed console when run with `cargo embed --example e05-lcd-st7789`.
#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use defmt as log;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::text::Text;
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::Rgb565;
use embedded_graphics_core::prelude::Point;
use embedded_graphics_core::prelude::RgbColor;
use embedded_graphics_core::Drawable;
use fugit::RateExtU32;
use hal::gpio;
use hal::gpio::FunctionSpi;
use hal::Clock;
use mipidsi::models::ST7789;
use mipidsi::ColorInversion;
use mipidsi::Orientation;
use rp2040_hal as hal;
use rp_pico as bsp;

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

    let cs = pins
        .gpio17
        .into_push_pull_output_in_state(gpio::PinState::Low);
    let dc = pins.gpio22.into_push_pull_output();
    let reset = pins
        .gpio21
        .into_push_pull_output_in_state(gpio::PinState::High);
    let _bl = pins
        .gpio20
        .into_push_pull_output_in_state(gpio::PinState::High);

    let _mosi = pins.gpio19.into_mode::<FunctionSpi>();
    let _miso = pins.gpio16.into_mode::<FunctionSpi>();
    let _clk = pins.gpio18.into_mode::<FunctionSpi>();

    let spi = hal::spi::Spi::<_, _, 8>::new(pac.SPI0);

    // let build_jmd_ips130_v20_no_cs = || {
    //     mipidsi::Builder::with_model(
    //         display_interface_spi::SPIInterfaceNoCS::new(
    //             spi.init(
    //                 &mut pac.RESETS,
    //                 clocks.peripheral_clock.freq(),
    //                 62u32.MHz(),
    //                 &embedded_hal::spi::MODE_3,
    //             ),
    //             dc,
    //         ),
    //         ST7789,
    //     )
    //     .with_color_order(mipidsi::ColorOrder::Rgb)
    //     .with_invert_colors(ColorInversion::Inverted)
    //     .with_orientation(Orientation::Portrait(false))
    //     .with_display_size(240, 240)
    // };

    let build_waveshare_320x240 = || {
        mipidsi::Builder::with_model(
            display_interface_spi::SPIInterface::new(
                spi.init(
                    &mut pac.RESETS,
                    clocks.peripheral_clock.freq(),
                    62u32.MHz(),
                    &embedded_hal::spi::MODE_3,
                ),
                dc,
                cs,
            ),
            ST7789,
        )
        .with_color_order(mipidsi::ColorOrder::Rgb)
        .with_invert_colors(ColorInversion::Inverted)
        .with_orientation(Orientation::Portrait(false))
        .with_display_size(320, 240)
    };

    let mut display = build_waveshare_320x240()
        .init(&mut delay, Some(reset))
        .unwrap();

    display.clear(Rgb565::MAGENTA).unwrap();

    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::YELLOW);

    Text::new("Hello, world!", Point::new(2, 28), style)
        .draw(&mut display)
        .unwrap();

    loop {
        cortex_m::asm::wfe();
    }
}
