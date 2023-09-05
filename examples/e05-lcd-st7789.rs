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
use embedded_graphics_core::prelude::Size;
use embedded_graphics_core::primitives::Rectangle;
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

    let _cs = pins
        .gpio17
        .into_push_pull_output_in_state(gpio::PinState::Low);
    let dc = pins.gpio22.into_push_pull_output();
    let reset = pins
        .gpio21
        .into_push_pull_output_in_state(gpio::PinState::High);
    let _bl = pins
        .gpio20
        .into_push_pull_output_in_state(gpio::PinState::High);

    let mosi = pins.gpio19.into_function::<FunctionSpi>();
    let miso = pins.gpio16.into_function::<FunctionSpi>();
    let sclk = pins.gpio18.into_function::<FunctionSpi>();

    let spi = hal::spi::Spi::<_, _, _, 8>::new(pac.SPI0, (mosi, miso, sclk));

    let mut display = mipidsi::Builder::with_model(
        display_interface_spi::SPIInterfaceNoCS::new(
            spi.init(
                &mut pac.RESETS,
                clocks.peripheral_clock.freq(),
                62u32.MHz(),
                &embedded_hal::spi::MODE_3,
            ),
            dc,
        ),
        ST7789,
    )
    .with_color_order(mipidsi::ColorOrder::Rgb)
    .with_invert_colors(ColorInversion::Inverted)
    .with_orientation(Orientation::Portrait(false))
    .with_display_size(240, 320)
    .with_framebuffer_size(240, 320)
    .init(&mut delay, Some(reset))
    .unwrap();

    let bg_color = Rgb565::new(4, 0, 4);
    display.clear(bg_color).unwrap();

    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::YELLOW);
    let text = "Hello, world!";
    let len = text.len() as u32;
    let mut text_bg_top_left = Point::new(2, 2);
    let mut text_top_left = Point::new(2, 22);
    let size = Size::new(len * 10, 22);
    for _ in 1..300 {
        Text::new(text, text_top_left, style)
            .draw(&mut display)
            .unwrap();
        cortex_m::asm::delay(610_000);

        display
            .fill_solid(&Rectangle::new(text_bg_top_left, size), bg_color)
            .unwrap();

        text_bg_top_left.y += 1;
        text_top_left.y += 1;
    }

    loop {
        cortex_m::asm::wfe();
    }
}
