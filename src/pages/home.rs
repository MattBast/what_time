use crate::components::{
    BackgroundBlur, InlineLi, IntroSubtitle, IntroTitle, Introtext, TimePicker, TimezoneSelect,
};
use crate::pages::Compare;
use crate::url_parse::url_query_to_timezones;
use crate::ZONE;
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

#[component]
pub fn Home() -> impl IntoView {
    // Watch the url query to decide whether to show the carousel or not.
    let (url_query, _set_url_query) = query_signal::<String>(ZONE);

    view! {
        <Show
            when=move || !url_query_to_timezones(url_query.get().unwrap_or_default()).is_empty()
            fallback=|| view! {
                <BackgroundBlur>
                    <div class="pt-24">
                        <WelcomeText/>
                    </div>
                </BackgroundBlur>
            }
        >
            <TimePicker/>
            <Compare/>
        </Show>

        <BackgroundBlur>
            <div class="py-8">
                // A select element that allows the user to add timezones to the carousel
                <TimezoneSelect/>
            </div>
        </BackgroundBlur>

    }
}

#[component]
fn WelcomeText() -> impl IntoView {
    view! {
        <Introtext>
            <IntroTitle>"Compare timezones, fast"</IntroTitle>
            <IntroSubtitle>
                <ul class="list-disc">
                    <InlineLi>"🙂 Pick your timezone."</InlineLi>
                    <InlineLi>"😀 Compare with another."</InlineLi>
                    <InlineLi>"😁 Keep adding more."</InlineLi>
                </ul>
            </IntroSubtitle>
        </Introtext>
    }
}
