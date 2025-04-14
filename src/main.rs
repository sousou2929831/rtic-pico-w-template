#![no_std]
#![no_main]

use defmt::{info, warn};
use defmt_rtt as _;
use panic_probe as _;
use rtic::app;
use rtic_monotonics::rp2040::prelude::*;
rp2040_timer_monotonic!(Mono);

#[app(device = rp_pico::hal::pac, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        let _mono = Mono::start(cx.device.TIMER, &mut cx.device.RESETS);
        info!("RTIC + defmt ready!");
        (Shared {}, Local {})
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        warn!("Entering idle loop...");
        loop {}
    }
}
