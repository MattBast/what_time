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
            <div class="py-8 hidden sm:block">
                // A select element that allows the user to add timezones to the carousel
                <TimezoneSelect/>
            </div>
        </BackgroundBlur>

        <FloatingButton/>

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

#[component]
fn FloatingButton() -> impl IntoView {
    view! {
        <div id="floating_button" class="fab block sm:hidden">
          <button class="btn btn-lg btn-circle btn-primary">
              <svg
                aria-label="New"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="2"
                stroke="currentColor"
                class="size-6"
              >
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
              </svg>
          </button>
        </div>
    }
}
