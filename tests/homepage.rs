use leptos::{prelude::*, task::tick};
use wasm_bindgen_test::*;
use what_time::pages::HomeContent;
use what_time::App;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn homepage_has_default_title() {
    mount_to_body(App);
    tick().await;

    let document = document();

    assert_eq!(document.title(), "What Time - Compare timezones, fast");
}

#[wasm_bindgen_test]
fn homepage_has_main_heading() {
    mount_to_body(|| {
        view! { <HomeContent
            timezones_query=Memo::new(move |_| None)
            set_timezones_query=SignalSetter::default()
            time_query=Memo::new(move |_| None)
            set_time_query=SignalSetter::default()
        /> }
    });

    let document = document();
    let h1 = document.query_selector("#main-heading").unwrap().unwrap();

    assert_eq!(h1.tag_name(), "H1");
    assert_eq!(
        h1.first_child().unwrap().text_content(),
        Some("Compare timezones, fast".to_string())
    );
}

#[wasm_bindgen_test]
fn homepage_has_sub_heading() {
    mount_to_body(|| {
        view! { <HomeContent
            timezones_query=Memo::new(move |_| None)
            set_timezones_query=SignalSetter::default()
            time_query=Memo::new(move |_| None)
            set_time_query=SignalSetter::default()
        /> }
    });

    let document = document();
    let ul = document.query_selector("#sub-headings").unwrap().unwrap();

    assert_eq!(ul.tag_name(), "UL");
    assert_eq!(
        ul.text_content(),
        Some("🙂 Pick your timezone.😀 Compare with another.😁 Keep adding more.".to_string())
    );
}
