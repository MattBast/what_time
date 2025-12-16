use crate::components::{
    BackgroundBlur, TimeInput, Timecard, TimecardDate, TimecardHeader, TimecardTime,
};
use crate::url_parse::url_query_to_time_increments;
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

        let mut timezones =
            url_query_to_time_increments(query.clone(), current_time.get_untracked());
        timezones.sort();

        // Add the timezones from url to the carousel.
        set_timezones.set(timezones);
    });

    view! {
        <div class="w-full">
            <BackgroundBlur>
                <div class="flex flex-wrap justify-center gap-2">
                    <For
                        each=move || get_timezones.get()
                        key=|timezone| timezone.timezone
                        children=move|timezone| {

                            view! {
                                <Timecard>
                                    <TimecardHeader>
                                        {move || timezone.display_header()}
                                    </TimecardHeader>

                                    <TimecardTime>
                                        <TimeInput
                                            current_time
                                            set_current_time
                                            timezone=timezone.timezone
                                        ></TimeInput>
                                    </TimecardTime>

                                    <TimecardDate>
                                        {move || timezone.display_date()}
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
