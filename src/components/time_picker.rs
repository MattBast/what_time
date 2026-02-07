use crate::components::Button;
use crate::timezone::utc_to_local_timezone;
use chrono::offset::LocalResult::Single;
use chrono::prelude::*;
use chrono::{DateTime, TimeDelta};
use chrono_tz::Tz;
use leptos::prelude::*;

#[component]
pub fn TimePicker(set_time_query: SignalSetter<Option<i64>>) -> impl IntoView {
    view! {
        <div class="flex w-full gap-5 justify-end content-end">
            <Button on:click=move |_| set_time_query.set(now_timestamp())>
                "Now"
            </Button>
        </div>
    }
}

fn now_timestamp() -> Option<i64> {
    Some(Utc::now().timestamp())
}

#[component]
pub fn TimeInput(
    time_query: Memo<Option<i64>>,
    set_time_query: SignalSetter<Option<i64>>,
    timezone: Tz,
) -> impl IntoView {
    let last_time = utc_to_local_timezone(time_query.get_untracked(), timezone);
    let (input_time, set_input_time) = signal(last_time.format("%H:%M").to_string());

    // Listen for the `current_time` url query to change and when it does, re-render the time in the inputs.
    Effect::new(move || {
        let now = utc_to_local_timezone(time_query.get(), timezone);
        set_input_time.set(now.format("%H:%M").to_string());
    });

    view! {
        <input
            class="input input-ghost input-xl text-5xl sm:text-6xl font-semibold w-fit max-w-full time-input-no-icon"
            type="time"
            name=format!("time_picker_{}", timezone.name().replace("/", "__"))
            id=format!("time_picker_{}", timezone.name().replace("/", "__"))
            prop:value=input_time
            on:input:target=move |ev| {
                let new_utc = update_current_time(ev.target().value(), time_query.get_untracked(), timezone);
                set_time_query.set(Some(new_utc))
            }
        />
    }
}

#[component]
pub fn DateInput(
    time_query: Memo<Option<i64>>,
    set_time_query: SignalSetter<Option<i64>>,
    timezone: Tz,
) -> impl IntoView {
    let (input_date, set_input_date) = signal(String::new());

    // Listen for the `current_time` url query to change and when it does, re-render the time in the inputs.
    Effect::new(move || {
        let now = utc_to_local_timezone(time_query.get(), timezone);
        set_input_date.set(now.format("%Y-%m-%d").to_string());
    });

    view! {
        <input
            class="input input-ghost input-sm max-w-full w-fit"
            type="date"
            name=format!("date_picker_{}", timezone.name().replace("/", "__"))
            id=format!("date_picker_{}", timezone.name().replace("/", "__"))
            prop:value=input_date
            on:input:target=move |ev| {
                let new_utc = update_current_date(ev.target().value(), time_query.get_untracked(), timezone);
                set_time_query.set(Some(new_utc))
            }
        />
    }
}

/// Get a date as string in this format: "%Y-%m-%d".
/// Figure out the offset datetime with the timezone, convert it to
/// UTC and return the UTC timestamp (formatted as a i64 number).
fn update_current_date(new_date: String, last_timestamp: Option<i64>, timezone: Tz) -> i64 {
    let last_date = utc_to_local_timezone(last_timestamp, timezone);

    let diff = get_date_diff(new_date, last_date, timezone);

    let utc = utc_to_local_timezone(last_timestamp, Tz::UTC);
    let new_utc = utc + diff;

    new_utc.timestamp()
}

/// Return the time difference between `new_date` and `last_date`.
/// It is assumed that `new_date` is at the same time as `last_date`.
fn get_date_diff(new_date: String, last_date: DateTime<Tz>, timezone: Tz) -> TimeDelta {
    if let Ok(naive_date) = NaiveDate::parse_from_str(&new_date, "%Y-%m-%d") {
        if let Single(new_date) = naive_date
            .and_time(last_date.time())
            .and_local_timezone(timezone)
        {
            return new_date - last_date;
        }
    };

    TimeDelta::zero()
}

/// Get a time as string in this format: "%H:%M".
/// Figure out the offset datetime with the timezone, convert it to
/// UTC and return the UTC timestamp (formatted as a i64 number).
fn update_current_time(new_date: String, last_timestamp: Option<i64>, timezone: Tz) -> i64 {
    let last_date = utc_to_local_timezone(last_timestamp, timezone);

    let diff = get_time_diff(new_date, last_date);

    let utc = utc_to_local_timezone(last_timestamp, Tz::UTC);
    let new_utc = utc + diff;

    new_utc.timestamp()
}

/// Return the time difference between `new_time` and `last_date`.
/// It is assumed that `new_time` is the time of the same day as `last_date`.
fn get_time_diff(new_time: String, last_date: DateTime<Tz>) -> TimeDelta {
    if let Ok(naive_time) = NaiveTime::parse_from_str(&new_time, "%H:%M") {
        if let Single(new_date) = last_date.with_time(naive_time) {
            return new_date - last_date;
        }
    };

    TimeDelta::zero()
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_add_one_minute_to_utc_from_gmt() {
        // The utc time is: 2025-12-17 10:45:00
        let new_utc =
            update_current_time("10:46".to_string(), Some(1765968316), Tz::Europe__London);

        assert_eq!(
            DateTime::from_timestamp(new_utc, 0).unwrap(),
            DateTime::parse_from_str("2025-12-17 10:46:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
    }

    #[test]
    fn test_add_one_minute_to_utc_from_cet() {
        // The utc time is: 2025-12-17 10:45:00
        let new_utc = update_current_time("10:46".to_string(), Some(1765968316), Tz::Europe__Paris);

        assert_eq!(
            DateTime::from_timestamp(new_utc, 0).unwrap(),
            DateTime::parse_from_str("2025-12-17 09:46:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
    }

    #[test]
    fn test_remove_one_minute_to_utc_from_gmt() {
        // The utc time is: 2025-12-17 10:45:00
        let new_utc =
            update_current_time("10:44".to_string(), Some(1765968316), Tz::Europe__London);

        assert_eq!(
            DateTime::from_timestamp(new_utc, 0).unwrap(),
            DateTime::parse_from_str("2025-12-17 10:44:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
    }

    #[test]
    fn test_add_one_hour_to_utc_from_gmt() {
        // The utc time is: 2025-12-17 10:45:00
        let new_utc =
            update_current_time("11:45".to_string(), Some(1765968316), Tz::Europe__London);

        assert_eq!(
            DateTime::from_timestamp(new_utc, 0).unwrap(),
            DateTime::parse_from_str("2025-12-17 11:45:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
    }

    #[test]
    fn test_add_one_hour_to_utc_from_cet() {
        // The utc time is: 2025-12-17 10:45:00
        let new_utc = update_current_time("11:45".to_string(), Some(1765968316), Tz::Europe__Paris);

        assert_eq!(
            DateTime::from_timestamp(new_utc, 0).unwrap(),
            DateTime::parse_from_str("2025-12-17 10:45:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
    }

    #[test]
    fn test_remove_one_hour_from_cet_to_move_utc_back_a_day() {
        // The utc time is: 2025-12-18 00:15:00
        let new_utc = update_current_time("00:15".to_string(), Some(1766016900), Tz::Europe__Paris);

        assert_eq!(
            DateTime::from_timestamp(new_utc, 0).unwrap(),
            DateTime::parse_from_str("2025-12-17 23:15:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
    }
}
