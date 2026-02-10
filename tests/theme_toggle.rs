use leptos::{prelude::*, task::tick};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, HtmlInputElement};
use what_time::components::ThemeToggle;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn theme_dropdown_contains_all_themes() {
    mount_to_body(ThemeToggle);

    let document = document();

    // Click dropdown button and wait for dropdown to open.
    let themes_button: HtmlElement = document
        .query_selector("#themes-button")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    themes_button.click();
    tick().await;

    // Get all themes from the dropdown list
    let inputs = document
        .query_selector_all("#themes-list > li > input")
        .unwrap();
    let mut themes_list = Vec::new();
    for i in 0..inputs.length() {
        let input: HtmlInputElement = inputs.item(i).unwrap().dyn_into().unwrap();

        themes_list.push(input.value());
    }

    assert_eq!(
        themes_list,
        vec![
            "default",
            "light",
            "dark",
            "cupcake",
            "bumblebee",
            "emerald",
            "corporate",
            "synthwave",
            "retro",
            "cyberpunk",
            "valentine",
            "halloween",
            "garden",
            "forest",
            "aqua",
            "lofi",
            "pastel",
            "fantasy",
            "wireframe",
            "black",
            "luxury",
            "dracula",
            "cmyk",
            "autumn",
            "business",
            "acid",
            "lemonade",
            "night",
            "coffee",
            "winter",
            "dim",
            "nord",
            "sunset",
            "caramellatte",
            "abyss",
            "silk"
        ]
    );
}
