#![no_std]
#![no_main]

use panic_probe as _;

#[rtic::app(device = rp_pico::hal::pac)]
mod app {
    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        (Shared {}, Local {}, init::Monotonics {})
    }
}
