use crate::components::{BackgroundBlur, DateInput, TimeInput, Timecard};
use crate::timezone::{sort_timezones, tz_to_city, tz_to_emoji, utc_to_local_timezone};
use crate::url_parse::url_query_to_timezones;
use chrono_tz::Tz;
use leptos::prelude::*;

#[component]
pub fn Compare(
    timezones_query: Memo<Option<String>>,
    time_query: Memo<Option<i64>>,
    set_time_query: SignalSetter<Option<i64>>,
) -> impl IntoView {
    let (get_timezones, set_timezones) = signal(Vec::new());

    // Listen for the `zone` url query to change and when it does, re-render the timezones.
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = timezones_query.get().unwrap_or_default();
        set_timezones.set(sorted_timezones_from_query(query));
    });

    view! {
        <BackgroundBlur>
            <div class="flex justify-center w-full overflow-x-auto">
                // The overscroll and touch-pan classes make scrolling on mobile smoother.
                <div class="carousel carousel-vertical sm:carousel-horizontal h-fit max-h-144 sm:h-auto">
                    <For
                        each=move || get_timezones.get()
                        key=|timezone| *timezone
                        children=move|timezone| {

                            let display_header = compare_card_header(time_query.get_untracked(), timezone);

                            view! {
                                <div class="carousel-item">
                                    <Timecard>

                                        <fieldset class="fieldset p-6 flex flex-col items-center">

                                            <label class="label text-2xl">{display_header}</label>

                                            <TimeInput
                                                time_query
                                                set_time_query
                                                timezone=timezone
                                            ></TimeInput>

                                            <DateInput
                                                time_query
                                                set_time_query
                                                timezone=timezone
                                            ></DateInput>

                                        </fieldset>

                                    </Timecard>
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </BackgroundBlur>
    }
}

fn sorted_timezones_from_query(query: String) -> Vec<Tz> {
    let mut timezones = url_query_to_timezones(query);
    sort_timezones(&mut timezones);
    timezones
}

fn compare_card_header(time_query: Option<i64>, timezone: Tz) -> String {
    let local_time = utc_to_local_timezone(time_query, timezone);
    format!(
        "{} {} ({})",
        tz_to_emoji(&timezone),
        tz_to_city(&timezone),
        local_time.format("%Z"),
    )
}

#[cfg(test)]
fn compare_time_input_value(time_query: Option<i64>, timezone: Tz) -> String {
    utc_to_local_timezone(time_query, timezone)
        .format("%H:%M")
        .to_string()
}

#[cfg(test)]
fn compare_date_input_value(time_query: Option<i64>, timezone: Tz) -> String {
    utc_to_local_timezone(time_query, timezone)
        .format("%Y-%m-%d")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_timezones_from_query_with_one_timezone() {
        let timezones = sorted_timezones_from_query("Europe__London".to_string());
        assert_eq!(timezones, vec![Tz::Europe__London]);
    }

    #[test]
    fn test_sorted_timezones_from_query_with_two_timezones() {
        let timezones = sorted_timezones_from_query("Asia__Calcutta,Asia__Shanghai".to_string());
        assert_eq!(timezones, vec![Tz::Asia__Calcutta, Tz::Asia__Shanghai]);
    }

    #[test]
    fn test_compare_card_header_uses_city_emoji_and_zone() {
        let header = compare_card_header(Some(1765920530), Tz::Europe__London);
        assert_eq!(header, "🇬🇧 London (GMT)");
    }

    #[test]
    fn test_compare_input_values_match_expected_local_times() {
        let timestamp = Some(1765987708);

        assert_eq!(
            compare_time_input_value(timestamp, Tz::Europe__London),
            "16:08"
        );
        assert_eq!(
            compare_date_input_value(timestamp, Tz::Europe__London),
            "2025-12-17"
        );

        assert_eq!(
            compare_time_input_value(timestamp, Tz::Europe__Paris),
            "17:08"
        );
        assert_eq!(
            compare_date_input_value(timestamp, Tz::Europe__Paris),
            "2025-12-17"
        );
    }
}
