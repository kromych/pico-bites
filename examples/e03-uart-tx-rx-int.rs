//! Shows off the minimal required structure.
//!
//! This will print "Running" in the embed console when run with `cargo embed --example e00-alive`.
#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use defmt as log;
use rp2040_hal as hal;
use rp_pico as bsp;

#[bsp::entry]
fn main() -> ! {
    log::info!(
        "Board {}, git revision {:x}, ROM verion {:x}",
        hal::rom_data::copyright_string(),
        hal::rom_data::git_revision(),
        hal::rom_data::rom_version_number(),
    );

    loop {
        cortex_m::asm::wfe();
    }
}
