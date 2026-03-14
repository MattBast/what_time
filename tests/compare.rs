use leptos::{prelude::*, task::tick};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Element, HtmlInputElement};
use what_time::pages::Compare;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn timezone_in_url_added_to_page() {
    let timezones_query = Memo::new(move |_| Some("Europe__London".to_string()));

    let document = document();
    let test_wrapper = document.create_element("section").unwrap();
    let _ = document.body().unwrap().append_child(&test_wrapper);

    // Render the Compare component
    let _dispose = mount_to(test_wrapper.clone().unchecked_into(), move || {
        view! {
            <Compare
                timezones_query=timezones_query
                time_query=Memo::new(move |_| None)
                set_time_query=SignalSetter::default()
            />
        }
    });

    tick().await;

    assert_eq!(get_timezones(test_wrapper), vec!["🇬🇧 London (GMT)"]);
}

#[wasm_bindgen_test]
async fn timezones_in_url_added_to_page() {
    let timezones_query = Memo::new(move |_| Some("Europe__London,Europe__Paris".to_string()));

    let document = document();
    let test_wrapper = document.create_element("section").unwrap();
    let _ = document.body().unwrap().append_child(&test_wrapper);

    // Render the Compare component
    let _dispose = mount_to(test_wrapper.clone().unchecked_into(), move || {
        view! {
            <Compare
                timezones_query=timezones_query
                time_query=Memo::new(move |_| None)
                set_time_query=SignalSetter::default()
            />
        }
    });

    tick().await;

    assert_eq!(
        get_timezones(test_wrapper),
        vec!["🇬🇧 London (GMT)", "🇫🇷 Paris (CET)"]
    );
}

#[wasm_bindgen_test]
async fn times_in_url_added_to_page() {
    let timezones_query = Memo::new(move |_| Some("Europe__London,Europe__Paris".to_string()));
    let time_query = Memo::new(move |_| Some(1765987708));

    let document = document();
    let test_wrapper = document.create_element("section").unwrap();
    let _ = document.body().unwrap().append_child(&test_wrapper);

    // Render the Compare component
    let _dispose = mount_to(test_wrapper.clone().unchecked_into(), move || {
        view! {
            <Compare
                timezones_query=timezones_query
                time_query=time_query
                set_time_query=SignalSetter::default()
            />
        }
    });

    // Sometimes multiple ticks are needed to see all the
    // elements in the DOM to populate
    tick().await;
    tick().await;
    tick().await;

    // London timezone card
    assert_eq!(
        get_input(&test_wrapper, "#time_picker_Europe__London"),
        "16:08"
    );
    assert_eq!(
        get_input(&test_wrapper, "#date_picker_Europe__London"),
        "2025-12-17"
    );

    // Paris timezone card
    assert_eq!(
        get_input(&test_wrapper, "#time_picker_Europe__Paris"),
        "17:08"
    );
    assert_eq!(
        get_input(&test_wrapper, "#date_picker_Europe__Paris"),
        "2025-12-17"
    );
}

fn get_timezones(test_wrapper: Element) -> Vec<String> {
    // Get all carousel items (timezones) present on the page
    let carousel_items = test_wrapper
        .query_selector_all("div.carousel-item")
        .unwrap();

    let mut timezones = Vec::new();
    for i in 0..carousel_items.length() {
        let input = carousel_items.item(i).unwrap().text_content().unwrap();

        timezones.push(input);
    }

    timezones
}

fn get_input(test_wrapper: &Element, id: &str) -> String {
    let input: HtmlInputElement = test_wrapper
        .query_selector(id)
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    // input.set_value("09:30");
    let time = input.value();

    time
}
