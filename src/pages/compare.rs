use crate::components::{BackgroundBlur, DateInput, TimeInput, Timecard};
use crate::timezone::{sort_timezones, tz_to_city, tz_to_emoji, utc_to_local_timezone};
use crate::url_parse::url_query_to_timezones;
use crate::{CURRENT_TIME, ZONE};
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

#[component]
pub fn Compare() -> impl IntoView {
    // Watch the url to decide which timezones to include and what times to compare.
    let (url_query, _set_url_query) = query_signal::<String>(ZONE);
    let (current_time, set_current_time) = query_signal::<i64>(CURRENT_TIME);

    let (get_timezones, set_timezones) = signal(Vec::new());

    // Listen for the `zone` url query to change and when it does, re-render the timezones.
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = url_query.get().unwrap_or_default();

        let mut timezones = url_query_to_timezones(query);
        sort_timezones(&mut timezones);

        // Add the timezones from url to the carousel.
        set_timezones.set(timezones);
    });

    view! {
        <BackgroundBlur>
            <div class="w-full overflow-x-auto">
                // The overscroll and touch-pan classes make scrolling on mobile smoother.
                <div class="carousel carousel-vertical sm:carousel-horizontal h-96 sm:h-auto">
                    <For
                        each=move || get_timezones.get()
                        key=|timezone| *timezone
                        children=move|timezone| {

                            let last_time = utc_to_local_timezone(current_time.get_untracked(), timezone);

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
                                                current_time
                                                set_current_time
                                                timezone=timezone
                                            ></TimeInput>

                                            <DateInput
                                                current_time
                                                set_current_time
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
