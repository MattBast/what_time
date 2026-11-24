use crate::timezone::utc_to_local_timezone;
use chrono_tz::Tz;
use leptos::html::div;
use leptos::prelude::*;

#[component]
pub fn TimeInput(time_query: Option<i64>, timezone: Tz) -> impl IntoView {
    let input_time = utc_to_local_timezone(time_query, timezone)
        .format("%H:%M")
        .to_string();

    div()
        .attr(
            "class",
            "text-5xl sm:text-6xl font-semibold w-fit max-w-full",
        )
        .child(input_time)
}

#[component]
pub fn DateInput(time_query: Option<i64>, timezone: Tz) -> impl IntoView {
    let input_date = utc_to_local_timezone(time_query, timezone)
        .format("%d/%m/%Y")
        .to_string();

    div()
        .attr("class", "text-sm max-w-full w-fit")
        .child(input_date)
}
