use leptos::{prelude::*, task::tick};
use wasm_bindgen_test::*;
use what_time::pages::Compare;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn timezone_in_url_added_to_page() {
    let timezones_query = Memo::new(move |_| Some("Europe__London".to_string()));
    document().body().unwrap().set_inner_html(""); //<-- reset the document body for test isolation

    mount_to_body(move || {
        view! { <Compare
            timezones_query=timezones_query
            time_query=Memo::new(move |_| None)
            set_time_query=SignalSetter::default()
        /> }
    });

    tick().await;

    assert_eq!(get_timezones(), vec!["🇬🇧 London (GMT)"]);
}

#[wasm_bindgen_test]
async fn timezones_in_url_added_to_page() {
    let timezones_query = Memo::new(move |_| Some("Europe__London,Europe__Paris".to_string()));
    document().body().unwrap().set_inner_html(""); //<-- reset the document body for test isolation

    mount_to_body(move || {
        view! { <Compare
            timezones_query=timezones_query
            time_query=Memo::new(move |_| None)
            set_time_query=SignalSetter::default()
        /> }
    });

    tick().await;

    assert_eq!(get_timezones(), vec!["🇬🇧 London (GMT)", "🇫🇷 Paris (CET)"]);
}

fn get_timezones() -> Vec<String> {
    let document = document();

    // Get all carousel items (timezones) present on the page
    let carousel_items = document.query_selector_all("div.carousel-item").unwrap();

    let mut timezones = Vec::new();
    for i in 0..carousel_items.length() {
        let input = carousel_items.item(i).unwrap().text_content().unwrap();

        timezones.push(input);
    }

    timezones
}
