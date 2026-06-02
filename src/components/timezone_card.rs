use crate::components::{DateInput, TimeInput};
use crate::timezone::{tz_to_city, tz_to_emoji, utc_to_local_timezone};
use chrono_tz::Tz;
use leptos::prelude::*;

#[component]
pub fn TimezoneCard(
    timezone: Tz,
    time_query: Memo<Option<i64>>,
    set_time_query: SignalSetter<Option<i64>>,
) -> impl IntoView {
    let header = move || timezone_card_header(time_query.get(), timezone);

    view! {
        <div class="card bg-base-100 border border-base-300 shadow-sm w-full sm:w-auto min-w-[14rem]">
            <div class="card-body items-center gap-4 p-6">
                <h2 class="card-title">{header}</h2>
                <TimeInput time_query set_time_query timezone />
                <DateInput time_query set_time_query timezone />
            </div>
        </div>
    }
}

/// City line shown at the top of each card (emoji, city, zone abbreviation).
pub(crate) fn timezone_card_header(time_query: Option<i64>, timezone: Tz) -> String {
    let local_time = utc_to_local_timezone(time_query, timezone);
    format!(
        "{} {} ({})",
        tz_to_emoji(&timezone),
        tz_to_city(&timezone),
        local_time.format("%Z"),
    )
}

/// Local time string bound to the card's time input (`HH:MM`).
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn timezone_card_local_time(time_query: Option<i64>, timezone: Tz) -> String {
    utc_to_local_timezone(time_query, timezone)
        .format("%H:%M")
        .to_string()
}

/// Local date string bound to the card's date input (`YYYY-MM-DD`).
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn timezone_card_local_date(time_query: Option<i64>, timezone: Tz) -> String {
    utc_to_local_timezone(time_query, timezone)
        .format("%Y-%m-%d")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fixed UTC instant used across several card tests (2025-12-17 16:08:28 UTC).
    const SAMPLE_TIMESTAMP: i64 = 1765987708;

    #[test]
    fn test_timezone_card_header_uses_city_emoji_and_zone_in_winter() {
        let header = timezone_card_header(Some(1765920530), Tz::Europe__London);
        assert_eq!(header, "🇬🇧 London (GMT)");
    }

    #[test]
    fn test_timezone_card_header_uses_bst_when_london_is_in_dst() {
        // 2025-06-15 14:00:00 UTC -> 15:00 BST
        let header = timezone_card_header(Some(1749996000), Tz::Europe__London);
        assert_eq!(header, "🇬🇧 London (BST)");
    }

    #[test]
    fn test_timezone_card_header_for_paris_shows_cet() {
        let header = timezone_card_header(Some(SAMPLE_TIMESTAMP), Tz::Europe__Paris);
        assert_eq!(header, "🇫🇷 Paris (CET)");
    }

    #[test]
    fn test_timezone_card_header_city_name_has_no_underscores() {
        let header = timezone_card_header(Some(SAMPLE_TIMESTAMP), Tz::America__Los_Angeles);
        assert_eq!(header, "🇺🇸 Los Angeles (PST)");
        assert!(!header.contains('_'));
    }

    #[test]
    fn test_timezone_card_local_time_and_date_for_london() {
        assert_eq!(
            timezone_card_local_time(Some(SAMPLE_TIMESTAMP), Tz::Europe__London),
            "16:08"
        );
        assert_eq!(
            timezone_card_local_date(Some(SAMPLE_TIMESTAMP), Tz::Europe__London),
            "2025-12-17"
        );
    }

    #[test]
    fn test_timezone_card_local_time_and_date_for_paris() {
        assert_eq!(
            timezone_card_local_time(Some(SAMPLE_TIMESTAMP), Tz::Europe__Paris),
            "17:08"
        );
        assert_eq!(
            timezone_card_local_date(Some(SAMPLE_TIMESTAMP), Tz::Europe__Paris),
            "2025-12-17"
        );
    }

    #[test]
    fn test_timezone_card_abidjan_values() {
        let timestamp = Some(1766076397);

        assert_eq!(
            timezone_card_header(timestamp, Tz::Africa__Abidjan),
            "🇨🇮 Abidjan (GMT)"
        );
        assert_eq!(
            timezone_card_local_time(timestamp, Tz::Africa__Abidjan),
            "16:46"
        );
        assert_eq!(
            timezone_card_local_date(timestamp, Tz::Africa__Abidjan),
            "2025-12-18"
        );
    }

    #[test]
    fn test_same_utc_instant_produces_different_card_times_per_zone() {
        let timestamp = Some(SAMPLE_TIMESTAMP);

        let london = timezone_card_local_time(timestamp, Tz::Europe__London);
        let paris = timezone_card_local_time(timestamp, Tz::Europe__Paris);

        assert_ne!(london, paris);
        assert_eq!(london, "16:08");
        assert_eq!(paris, "17:08");
    }

    #[test]
    fn test_card_shows_different_calendar_dates_for_same_utc_instant() {
        // London 23:40 on 2025-12-18 is 00:40 on 2025-12-19 in Paris (see time_picker tests).
        let timestamp = Some(1766101200);

        assert_eq!(
            timezone_card_local_time(timestamp, Tz::Europe__London),
            "23:40"
        );
        assert_eq!(
            timezone_card_local_date(timestamp, Tz::Europe__London),
            "2025-12-18"
        );
        assert_eq!(
            timezone_card_local_time(timestamp, Tz::Europe__Paris),
            "00:40"
        );
        assert_eq!(
            timezone_card_local_date(timestamp, Tz::Europe__Paris),
            "2025-12-19"
        );
    }
}
