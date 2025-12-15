use crate::components::{BackgroundBlur, Timecard, TimecardDate, TimecardHeader, TimecardTime};
use crate::timezone::TimeIncrement;
use crate::url_parse::url_query_to_timezones;
use crate::{CURRENT_TIME, ZONE};
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

#[component]
pub fn Compare() -> impl IntoView {
    // Watch the url to decide which timezones to include and what times to compare.
    let (url_query, _set_url_query) = query_signal::<String>(ZONE);
    let (current_time, _set_current_time) = query_signal::<i64>(CURRENT_TIME);

    let (get_timezones, set_timezones) = signal(Vec::new());

    // Listen for the `zone` url query to change and when it does, re-render the timezones.
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = url_query.get().unwrap_or_default();

        set_timezones.set(url_query_to_timezones(query.clone()));
    });

    view! {
        <div class="w-full">
            <BackgroundBlur>
                <div class="flex flex-wrap justify-center gap-2">
                    <For
                        each=move || get_timezones.get()
                        key=|timezone| timezone.clone()
                        children=move|timezone| {
                            let hour = move || match current_time.get() {
                                Some(timestamp) => TimeIncrement::from_timestamp(timestamp, timezone.clone()),
                                None => TimeIncrement::now(timezone.clone()),
                            };

                            view! {
                                <Timecard large=true>
                                    <TimecardHeader>
                                        {move || hour().display_header()}
                                    </TimecardHeader>
                                    <TimecardTime>
                                        {move || hour().display_time()}
                                    </TimecardTime>
                                    <TimecardDate>
                                        {move || hour().display_date()}
                                    </TimecardDate>
                                </Timecard>
                            }
                        }
                    />
                </div>
            </BackgroundBlur>
        </div>
    }
}
