#![allow(non_snake_case)]
#![deny(clippy::unwrap_used)]

use leptos::mount::mount_to_body;
use what_time::*;

fn main() {
    // If running without the `--release` flag, use debug level logging.
    // I.e. use debug in non-prod and info in prod.
    let log_level = if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Info
    };

    let _ = console_log::init_with_level(log_level);
    console_error_panic_hook::set_once();

    mount_to_body(App)
}
