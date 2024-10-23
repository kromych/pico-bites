//! This example uses an Waveshare RP2040 Matrix board.
//!
//! The LEDs are connected to GPIO 16: https://www.waveshare.com/wiki/RP2040-Matrix#![no_std]
#![no_std]
#![no_main]

use panic_halt as _;
use smart_leds::brightness;
use smart_leds::SmartLedsWrite;
use smart_leds::RGB8;
use waveshare_rp2040_zero::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pio::PIOExt,
        timer::Timer,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use ws2812_pio::Ws2812;

const STRIP_LEN: usize = 25;

#[waveshare_rp2040_zero::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut frame_delay =
        cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut ws = Ws2812::new(
        // The onboard NeoPixel is attached to GPIO pin #16 on the Feather RP2040.
        pins.neopixel.into_function(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut leds: [RGB8; STRIP_LEN] = [(0, 128, 0).into(); STRIP_LEN];

    // Bring down the overall brightness of the strip to not blow
    // the USB power supply: every LED draws ~60mA, RGB means 3 LEDs per
    // ws2812 LED, for 3 LEDs that would be: 3 * 3 * 60mA, which is
    // already 540mA for just 3 white LEDs!
    let strip_brightness = 1u8; // Limit brightness to 1/256
    let hello = b"* WHAT'S UP, WORLD? * ";
    let mut iter_num = 0;
    let mut char_num = 0;
    let mut time = 0f32;
    let animation_speed = 0.1;

    loop {
        // Prepare frame
        let fg_color = color::get_color(time);
        let bg_color = &(0, 0, 0).into();
        render_ascii(&mut leds, hello[char_num], &fg_color, Some(bg_color));

        // Write frame
        ws.write(brightness(leds.iter().copied(), strip_brightness))
            .unwrap();

        iter_num += 1;
        char_num = iter_num / 30 % hello.len();

        // Increase the time counter variable and make sure it
        // stays inbetween 0.0 to 1.0 range.
        time += (16.0 / 1000.0) * animation_speed;
        while time > 1.0 {
            time -= 1.0;
        }

        // Wait a bit until calculating the next frame.
        frame_delay.delay_ms(16);
    }
}

fn render_ascii(
    leds: &mut [RGB8; STRIP_LEN],
    ascii_sym: u8,
    fg_color: &RGB8,
    bg_color: Option<&RGB8>,
) {
    // Trading space for absence of the conditional statements.
    const FONT_5X5: [[u8; 5]; 256] = [
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b11111, 0b00000, 0b11111, 0b00000], // non-printable
        [0b11111, 0b00000, 0b11111, 0b00000, 0b11111], // non-printable
        [0b00000, 0b00000, 0b00000, 0b00000, 0b11111], // non-printable
        // ASCII 32-127
        [0b00000, 0b00000, 0b00000, 0b00000, 0b00000], // (space)
        [0b00100, 0b00100, 0b00100, 0b00000, 0b00100], // !
        [0b01010, 0b01010, 0b00000, 0b00000, 0b00000], // "
        [0b01010, 0b11111, 0b01010, 0b11111, 0b01010], // #
        [0b00100, 0b01111, 0b10100, 0b01011, 0b00100], // $
        [0b11001, 0b11010, 0b00100, 0b01011, 0b10011], // %
        [0b01100, 0b10010, 0b10101, 0b01000, 0b10100], // &
        [0b00100, 0b00100, 0b00000, 0b00000, 0b00000], // '
        [0b00010, 0b00100, 0b01000, 0b00100, 0b00010], // (
        [0b01000, 0b00100, 0b00010, 0b00100, 0b01000], // )
        [0b00100, 0b10101, 0b01110, 0b10101, 0b00100], // *
        [0b00000, 0b00100, 0b01110, 0b00100, 0b00000], // +
        [0b00000, 0b00000, 0b00000, 0b00100, 0b01000], // ,
        [0b00000, 0b00000, 0b01110, 0b00000, 0b00000], // -
        [0b00000, 0b00000, 0b00000, 0b00100, 0b00000], // .
        [0b00001, 0b00010, 0b00100, 0b01000, 0b10000], // /
        [0b01110, 0b10001, 0b10001, 0b10001, 0b01110], // 0
        [0b00100, 0b01100, 0b00100, 0b00100, 0b01110], // 1
        [0b01110, 0b10001, 0b00110, 0b01000, 0b11111], // 2
        [0b11111, 0b00010, 0b01110, 0b00001, 0b11110], // 3
        [0b10001, 0b10001, 0b11111, 0b00001, 0b00001], // 4
        [0b11111, 0b10000, 0b11110, 0b00001, 0b11110], // 5
        [0b01110, 0b10000, 0b11110, 0b10001, 0b01110], // 6
        [0b11111, 0b00001, 0b00010, 0b00100, 0b01000], // 7
        [0b01110, 0b10001, 0b01110, 0b10001, 0b01110], // 8
        [0b01110, 0b10001, 0b01111, 0b00001, 0b01110], // 9
        [0b00000, 0b00100, 0b00000, 0b00100, 0b00000], // :
        [0b00000, 0b00100, 0b00000, 0b00100, 0b01000], // ;
        [0b00010, 0b00100, 0b01000, 0b00100, 0b00010], // <
        [0b00000, 0b01110, 0b00000, 0b01110, 0b00000], // =
        [0b01000, 0b00100, 0b00010, 0b00100, 0b01000], // >
        [0b01110, 0b10001, 0b00010, 0b00000, 0b00100], // ?
        [0b01110, 0b10001, 0b10111, 0b10101, 0b01110], // @
        [0b01110, 0b10001, 0b11111, 0b10001, 0b10001], // A
        [0b11110, 0b10001, 0b11110, 0b10001, 0b11110], // B
        [0b01110, 0b10001, 0b10000, 0b10001, 0b01110], // C
        [0b11100, 0b10010, 0b10001, 0b10010, 0b11100], // D
        [0b11111, 0b10000, 0b11100, 0b10000, 0b11111], // E
        [0b11111, 0b10000, 0b11100, 0b10000, 0b10000], // F
        [0b01110, 0b10001, 0b10000, 0b10100, 0b01110], // G
        [0b10001, 0b10001, 0b11111, 0b10001, 0b10001], // H
        [0b01110, 0b00100, 0b00100, 0b00100, 0b01110], // I
        [0b00111, 0b00010, 0b00010, 0b10010, 0b01100], // J
        [0b10001, 0b10010, 0b11000, 0b10010, 0b10001], // K
        [0b10000, 0b10000, 0b10000, 0b10000, 0b11111], // L
        [0b10001, 0b11011, 0b10101, 0b10001, 0b10001], // M
        [0b10001, 0b11001, 0b10101, 0b10011, 0b10001], // N
        [0b01110, 0b10001, 0b10001, 0b10001, 0b01110], // O
        [0b11110, 0b10001, 0b11110, 0b10000, 0b10000], // P
        [0b01110, 0b10001, 0b10101, 0b10010, 0b01101], // Q
        [0b11110, 0b10001, 0b11110, 0b10010, 0b10001], // R
        [0b01110, 0b10000, 0b01110, 0b00001, 0b01110], // S
        [0b11111, 0b00100, 0b00100, 0b00100, 0b00100], // T
        [0b10001, 0b10001, 0b10001, 0b10001, 0b01110], // U
        [0b10001, 0b10001, 0b01010, 0b01010, 0b00100], // V
        [0b10001, 0b10001, 0b10101, 0b11011, 0b10001], // W
        [0b10001, 0b01010, 0b00100, 0b01010, 0b10001], // X
        [0b10001, 0b10001, 0b01110, 0b00100, 0b00100], // Y
        [0b11111, 0b00010, 0b00100, 0b01000, 0b11111], // Z
        [0b01110, 0b01000, 0b01000, 0b01000, 0b01110], // [
        [0b10000, 0b01000, 0b00100, 0b00010, 0b00001], // \
        [0b01110, 0b00010, 0b00010, 0b00010, 0b01110], // ]
        [0b00100, 0b01010, 0b10001, 0b00000, 0b00000], // ^
        [0b00000, 0b00000, 0b00000, 0b00000, 0b11111], // _
        [0b00100, 0b00100, 0b00010, 0b00000, 0b00000], // `
        [0b00000, 0b00110, 0b01000, 0b01110, 0b10001], // a
        [0b10000, 0b10000, 0b11100, 0b10010, 0b11100], // b
        [0b00000, 0b01100, 0b10000, 0b10000, 0b01100], // c
        [0b00010, 0b00010, 0b01110, 0b10010, 0b01110], // d
        [0b00000, 0b01100, 0b10100, 0b11000, 0b01100], // e
        [0b00100, 0b01010, 0b01100, 0b01000, 0b01000], // f
        [0b00000, 0b01110, 0b10010, 0b01110, 0b00010], // g
        [0b10000, 0b10000, 0b11100, 0b10010, 0b10010], // h
        [0b00100, 0b00000, 0b00100, 0b00100, 0b00100], // i
        [0b00010, 0b00000, 0b00010, 0b00010, 0b10010], // j
        [0b10000, 0b10010, 0b10100, 0b11000, 0b10100], // k
        [0b00100, 0b00100, 0b00100, 0b00100, 0b00100], // l
        [0b00000, 0b11010, 0b10101, 0b10101, 0b10101], // m
        [0b00000, 0b11100, 0b10010, 0b10010, 0b10010], // n
        [0b00000, 0b01100, 0b10010, 0b10010, 0b01100], // o
        [0b00000, 0b11100, 0b10010, 0b11100, 0b10000], // p
        [0b00000, 0b01110, 0b10010, 0b01110, 0b00010], // q
        [0b00000, 0b10100, 0b11000, 0b10000, 0b10000], // r
        [0b00000, 0b01100, 0b00100, 0b00010, 0b01100], // s
        [0b01000, 0b11100, 0b01000, 0b01000, 0b00100], // t
        [0b00000, 0b10010, 0b10010, 0b10010, 0b01110], // u
        [0b00000, 0b10001, 0b10001, 0b01010, 0b00100], // v
        [0b00000, 0b10001, 0b10101, 0b10101, 0b01010], // w
        [0b00000, 0b10001, 0b01010, 0b01010, 0b10001], // x
        [0b00000, 0b10010, 0b10010, 0b01110, 0b00010], // y
        [0b00000, 0b11110, 0b00100, 0b01000, 0b11110], // z
        [0b00100, 0b01000, 0b01000, 0b01000, 0b00100], // [
        [0b00100, 0b00100, 0b00000, 0b00100, 0b00100], // |
        [0b01000, 0b00100, 0b00100, 0b00100, 0b01000], // }
        [0b00000, 0b00000, 0b01010, 0b10100, 0b00000], // ~
        // Extended, not impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b10101, 0b10101, 0b10101, 0b10101, 0b10101], // non impelemented
        [0b01010, 0b01010, 0b01010, 0b01010, 0b01010], // non impelemented
        [0b11111, 0b11111, 0b11111, 0b11111, 0b11111], // non impelemented
    ];

    let glyph = &FONT_5X5[ascii_sym as usize];
    let mut led_idx = 0;
    for &scan_line in glyph.iter().rev() {
        for pixel_idx in 0..5 {
            if scan_line & 1u8.wrapping_shl(pixel_idx) != 0 {
                leds[led_idx] = *fg_color;
            } else if let Some(bg_color) = bg_color {
                leds[led_idx] = *bg_color;
            }
            led_idx += 1;
        }
    }
}

// Based on the official SDK example
mod color {
    use smart_leds::RGB8;

    pub fn get_color(t: f32) -> RGB8 {
        // Import the `sin` function for a smooth hue animation from the
        // Pico rp2040 ROM.
        let sin = waveshare_rp2040_zero::hal::rom_data::float_funcs::fsin::ptr();

        let sin_11 = sin((t) * 2.0 * core::f32::consts::PI);
        // Bring -1..1 sine range to 0..1 range:
        let sin_01 = (sin_11 + 1.0) * 0.5;

        let hue = 360.0 * sin_01;
        let sat = 1.0;
        let val = 1.0;

        let rgb = hsv2rgb_u8(hue, sat, val);
        rgb.into()
    }

    pub fn hsv2rgb(hue: f32, sat: f32, val: f32) -> (f32, f32, f32) {
        let c = val * sat;
        let v = (hue / 60.0) % 2.0 - 1.0;
        let v = if v < 0.0 { -v } else { v };
        let x = c * (1.0 - v);
        let m = val - c;
        let (r, g, b) = if hue < 60.0 {
            (c, x, 0.0)
        } else if hue < 120.0 {
            (x, c, 0.0)
        } else if hue < 180.0 {
            (0.0, c, x)
        } else if hue < 240.0 {
            (0.0, x, c)
        } else if hue < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        (r + m, g + m, b + m)
    }

    pub fn hsv2rgb_u8(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
        let r = hsv2rgb(h, s, v);

        (
            (r.0 * 255.0) as u8,
            (r.1 * 255.0) as u8,
            (r.2 * 255.0) as u8,
        )
    }
}
