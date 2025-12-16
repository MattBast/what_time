use crate::components::Button;
use crate::timezone::TimeIncrement;
use crate::CURRENT_TIME;
use chrono::format::ParseResult;
use chrono::offset::LocalResult::Single;
use chrono::prelude::*;
use chrono::{DateTime, TimeDelta};
use chrono_tz::Tz;
use leptos::ev::{Event, Targeted};
use leptos::prelude::*;
use leptos::web_sys::HtmlInputElement;
use leptos_router::hooks::query_signal;
use log::info;

#[component]
pub fn TimePicker() -> impl IntoView {
    // Watch the url query to decide whether to show the carousel or not.
    let (_current_time, set_current_time) = query_signal::<i64>(CURRENT_TIME);

    view! {
        <div class="flex w-full gap-5 justify-end content-end">
            <Button
                on:click=move |_| {
                    let dt = TimeIncrement::now(Tz::UCT);
                    set_current_time.set(Some(dt.timestamp()));
                }
            >
                "Now"
            </Button>
        </div>
    }
}

#[component]
pub fn TimeInput(
    current_time: Memo<Option<i64>>,
    set_current_time: SignalSetter<Option<i64>>,
    timezone: Tz,
) -> impl IntoView {
    let last_time = utc_to_local_timezone(current_time.get_untracked(), timezone);
    let (input_time, set_input_time) = signal(last_time.format("%H:%M").to_string());

    // Listen for the `current_time` url query to change and when it does, re-render the time in the inputs.
    Effect::new(move || {
        let now = utc_to_local_timezone(current_time.get(), timezone);
        set_input_time.set(now.format("%H:%M").to_string());
    });

    view! {
        <input
            class="
                rounded-md text-zinc-900 dark:text-zinc-100 text-center
                focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-teal-500
                dark:[color-scheme:dark] cursor-pointer
            "
            type="time"
            name="time-picker"
            prop:value=input_time
            on:input:target=move |ev| {
                let new_utc = update_current_time(ev.target().value(), current_time.get_untracked(), timezone);
                set_current_time.set(Some(new_utc))
            }
        />
    }
}

#[component]
pub fn DateInput(
    current_time: Memo<Option<i64>>,
    set_current_time: SignalSetter<Option<i64>>,
    timezone: Tz,
) -> impl IntoView {
    let (input_date, set_input_date) = signal(String::new());

    // Listen for the `current_time` url query to change and when it does, re-render the time in the inputs.
    Effect::new(move || {
        let now = utc_to_local_timezone(current_time.get(), timezone);
        set_input_date.set(now.format("%Y-%m-%d").to_string());
    });

    view! {
        <input
            class="
                rounded-md text-zinc-900 dark:text-zinc-100 text-center
                focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-teal-500
                dark:[color-scheme:dark] cursor-pointer
            "
            type="date"
            name="date-picker"
            prop:value=input_date
            on:input:target=move |ev| {
                let new_utc = update_current_date(ev.target().value(), current_time.get_untracked(), timezone);
                set_current_time.set(Some(new_utc))
            }
        />
    }
}

/// Get an event that contains a date in this format: "%Y-%m-%d".
/// Figure out the offset datetime with the timezone, convert it to
/// UTC and return the UTC timestamp (formatted as a i64 number).
fn update_current_date(ev: String, current_time: Option<i64>, timezone: Tz) -> i64 {
    let last_date = utc_to_local_timezone(current_time, timezone);

    let diff = get_date_diff(ev, last_date, timezone);

    let utc = utc_to_local_timezone(current_time, Tz::UTC);
    let new_utc = utc + diff;

    new_utc.timestamp()
}

fn update_current_time(ev: String, current_time: Option<i64>, timezone: Tz) -> i64 {
    let last_date = utc_to_local_timezone(current_time, timezone);

    let diff = get_time_diff(ev, last_date, timezone);

    let utc = utc_to_local_timezone(current_time, Tz::UTC);
    let new_utc = utc + diff;

    new_utc.timestamp()
}

fn get_date_diff(ev: String, last_date: DateTime<Tz>, timezone: Tz) -> TimeDelta {
    if let Ok(naive_date) = event_to_date(ev) {
        if let Single(new_date) = naive_date
            .and_time(last_date.time())
            .and_local_timezone(timezone)
        {
            return new_date - last_date;
        }
    };

    TimeDelta::zero()
}

fn get_time_diff(ev: String, last_date: DateTime<Tz>, timezone: Tz) -> TimeDelta {
    if let Ok(naive_time) = event_to_time(ev) {
        if let Single(new_date) = last_date.with_time(naive_time) {
            return new_date - last_date;
        }
    };

    TimeDelta::zero()
}

/// Get a timestamp with the UTC timezone. Convert it to the specified timezone.
/// If `current_time` is None, default the time to now.
fn utc_to_local_timezone(current_time: Option<i64>, tz: Tz) -> DateTime<Tz> {
    match current_time {
        Some(timestamp) => DateTime::from_timestamp(timestamp, 0)
            .unwrap_or_default()
            .with_timezone(&tz),
        None => Utc::now().with_timezone(&tz),
    }
}

fn event_to_time(ev: String) -> ParseResult<NaiveTime> {
    NaiveTime::parse_from_str(&ev, "%H:%M")
}

fn event_to_date(ev: String) -> ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(&ev, "%Y-%m-%d")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_gmt() {
        let gmt_time = utc_to_local_timezone(Some(1765920530), Tz::Europe__London);

        assert_eq!(
            gmt_time,
            DateTime::parse_from_str("2025-12-16 21:28:50 +0000", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .with_timezone(&Tz::Europe__London)
        );
    }

    #[test]
    fn test_parse_to_cet() {
        let gmt_time = utc_to_local_timezone(Some(1765920530), Tz::Europe__Paris);

        assert_eq!(
            gmt_time,
            DateTime::parse_from_str("2025-12-16 22:28:50 +0100", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .with_timezone(&Tz::Europe__Paris)
        );
    }

    #[test]
    fn test_parse_to_ist() {
        let gmt_time = utc_to_local_timezone(Some(1765920530), Tz::Asia__Calcutta);

        assert_eq!(
            gmt_time,
            DateTime::parse_from_str("2025-12-17 02:58:50 +0530", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .with_timezone(&Tz::Asia__Calcutta)
        );
    }

    #[test]
    fn test_add_gmt_day() {
        // The utc time is: 2025-12-30 22:28:00
        let new_utc = update_current_date(
            "2025-12-31".to_string(),
            Some(1767133680),
            Tz::Europe__London,
        );

        assert_eq!(
            DateTime::from_timestamp(new_utc, 0).unwrap(),
            DateTime::parse_from_str("2025-12-31 22:28:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
    }

    #[test]
    fn test_add_cet_day() {
        // The utc time is: 2025-12-30 22:28:00
        let new_utc = update_current_date(
            "2025-12-31".to_string(),
            Some(1767133680),
            Tz::Europe__Paris,
        );

        assert_eq!(
            DateTime::from_timestamp(new_utc, 0).unwrap(),
            DateTime::parse_from_str("2025-12-31 22:28:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
    }

    #[test]
    fn test_move_to_new_year() {
        // The utc time is: 2025-12-31 22:28:00
        let new_utc = update_current_date(
            "2026-01-01".to_string(),
            Some(1767220080),
            Tz::Europe__Paris,
        );

        assert_eq!(
            DateTime::from_timestamp(new_utc, 0).unwrap(),
            DateTime::parse_from_str("2026-01-01 22:28:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
    }

    #[test]
    fn test_add_two_cet_days() {
        // The utc time is: 2025-12-29 22:28:00
        let new_utc = update_current_date(
            "2025-12-31".to_string(),
            Some(1767047280),
            Tz::Europe__Paris,
        );

        assert_eq!(
            DateTime::from_timestamp(new_utc, 0).unwrap(),
            DateTime::parse_from_str("2025-12-31 22:28:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
    }

    #[test]
    fn test_remove_gmt_day() {
        // The utc time is: 2025-12-30 22:28:00
        let new_utc = update_current_date(
            "2025-12-29".to_string(),
            Some(1767133680),
            Tz::Europe__London,
        );

        assert_eq!(
            DateTime::from_timestamp(new_utc, 0).unwrap(),
            DateTime::parse_from_str("2025-12-29 22:28:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
    }

    #[test]
    fn test_move_back_a_year() {
        // The utc time is: 2026-01-01 22:28:00
        let new_utc = update_current_date(
            "2025-12-31".to_string(),
            Some(1767306480),
            Tz::Europe__Paris,
        );

        assert_eq!(
            DateTime::from_timestamp(new_utc, 0).unwrap(),
            DateTime::parse_from_str("2025-12-31 22:28:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
    }
}
