pub mod app;
pub mod buttons;
pub mod carousel;
pub mod timecard;
pub mod timezone;
pub mod timezone_select;
pub mod url_parse;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
