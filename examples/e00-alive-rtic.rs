#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

#[rtic::app(device = rp_pico::hal::pac, peripherals = true)]
mod app {
    use defmt as log;
    use rp2040_hal as hal;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        log::info!(
            "Board {}, git revision {:x}, ROM verion {:x}",
            hal::rom_data::copyright_string(),
            hal::rom_data::git_revision(),
            hal::rom_data::rom_version_number(),
        );
        log::info!("RTIC app init");

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            log::info!("RTIC idle");
            cortex_m::asm::wfe();
        }
    }
}
