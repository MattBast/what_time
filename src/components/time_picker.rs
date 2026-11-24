use crate::timezone::utc_to_local_timezone;
use chrono::Timelike;
use chrono_tz::Tz;
use leptos::html::{div, span};
use leptos::prelude::*;

#[component]
pub fn TimeInput(time_query: ReadSignal<i64>, timezone: Tz) -> impl IntoView {
    let hours = move || utc_to_local_timezone(time_query.get(), timezone).hour();

    let minutes = move || utc_to_local_timezone(time_query.get(), timezone).minute();

    span()
        .attr(
            "class",
            "countdown text-5xl sm:text-6xl font-semibold w-fit max-w-full",
        )
        .child((
            span()
                .attr("aria-live", "polite")
                .attr("aria-label", move || format!("{:02}", hours()))
                .attr("style", move || {
                    format!("--value:{}; --digits: 2;", hours())
                })
                .child(move || format!("{:02}", hours())),
            ":",
            span()
                .attr("aria-live", "polite")
                .attr("aria-label", move || format!("{:02}", minutes()))
                .attr("style", move || {
                    format!("--value:{}; --digits: 2;", minutes())
                })
                .child(move || format!("{:02}", minutes())),
        ))
}

#[component]
pub fn DateInput(time_query: ReadSignal<i64>, timezone: Tz) -> impl IntoView {
    let input_date = move || {
        utc_to_local_timezone(time_query.get(), timezone)
            .format("%d/%m/%Y")
            .to_string()
    };

    div()
        .attr("class", "text-sm max-w-full w-fit")
        .child(input_date)
}
