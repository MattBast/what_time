use crate::components::{BackgroundBlur, DateInput, TimeInput, Timecard};
use crate::timezone::{sort_timezones, tz_to_city, tz_to_emoji, utc_to_local_timezone};
use crate::url_parse::url_query_to_timezones;
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

        let mut timezones = url_query_to_timezones(query);
        sort_timezones(&mut timezones);

        // Add the timezones from url to the carousel.
        set_timezones.set(timezones);
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

                            let last_time = utc_to_local_timezone(time_query.get_untracked(), timezone);

                            let display_header = format!(
                                "{} {} ({})",
                                tz_to_emoji(&timezone),
                                tz_to_city(&timezone),
                                last_time.format("%Z"),
                            );

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
