#![allow(non_snake_case)]
#![deny(clippy::unwrap_used)]

use leptos::mount::mount_to_body;
use what_time::*;

fn main() {
    // 🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨
    // Make the logging level configurable
    // 🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App)
}
