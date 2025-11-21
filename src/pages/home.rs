use crate::components::{InlineLi, IntroSubtitle, IntroTitle, Introtext, TimezoneSelect};
use crate::pages::CompareInner;
use crate::url_parse::url_query_to_time_increments;
use crate::ZONE;
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

#[component]
pub fn Home() -> impl IntoView {
    // Watch the url query to decide whether to show the carousel or not.
    let (url_query, _set_url_query) = query_signal::<String>(ZONE);

    view! {
        // ---------------------------------------------------------------------------------
        // An experiement to make the text and timezones fade in and out
        // ---------------------------------------------------------------------------------
        // <div
        //     // Hide the page title and description when there are timezones in the url.
        //     class="transition-all duration-700 ease-out origin-top"
        //     class=(["opacity-0", "-translate-y-2"], move || !url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
        //     class=(["opacity-100", "translate-y-0"], move || url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
        // >
        //     <WelcomeText/>
        // </div>

        // <div
        //     // Hide the page title and description when there are timezones in the url.
        //     class="transition-all duration-700 ease-out origin-top"
        //     class=(["opacity-0", "-translate-y-2"], move || url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
        //     class=(["opacity-100", "translate-y-0"], move || !url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
        // >
        //     <CompareInner/>
        // </div>

        <Show
            when=move || !url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty()
            fallback=|| view! {
                <BackgroundBlur>
                    <div class="pt-24">
                        <WelcomeText/>
                    </div>
                </BackgroundBlur>
            }
        >
            <BackgroundBlur>
                <div class="pt-24">
                    <CompareInner/>
                </div>
            </BackgroundBlur>
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
            <IntroTitle>"Compare timezones, quickly"</IntroTitle>
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

#[component]
fn BackgroundBlur(children: Children) -> impl IntoView {
    view! {
        <div class="bg-radial from-white dark:from-black from-20%">
            {children()}
        </div>
    }
}
