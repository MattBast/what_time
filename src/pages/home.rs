use crate::components::{
    BackgroundBlur, InlineLi, IntroSubtitle, IntroTitle, Introtext, TimePicker,
    TimezoneDrawerContent, TimezoneSelect,
};
use crate::pages::Compare;
use crate::url_parse::url_query_to_timezones;
use crate::{CURRENT_TIME, ZONE};
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

#[component]
pub fn Home() -> impl IntoView {
    // Watch the url queries
    let (timezones_query, set_timezones_query) = query_signal::<String>(ZONE);
    let (time_query, set_time_query) = query_signal::<i64>(CURRENT_TIME);

    view! { <HomeContent timezones_query set_timezones_query time_query set_time_query/> }
}

#[component]
pub fn HomeContent(
    timezones_query: Memo<Option<String>>,
    set_timezones_query: SignalSetter<Option<String>>,
    time_query: Memo<Option<i64>>,
    set_time_query: SignalSetter<Option<i64>>,
) -> impl IntoView {
    view! {
        <TimezoneDrawer timezones_query set_timezones_query>
            <Show
                when=move || !url_query_to_timezones(timezones_query.get().unwrap_or_default()).is_empty()
                fallback=|| view! {
                    <BackgroundBlur>
                        <div class="pt-24">
                            <WelcomeText/>
                        </div>
                    </BackgroundBlur>
                }
            >
                <TimePicker set_time_query/>
                <Compare timezones_query time_query set_time_query/>
            </Show>

            // Select component for tablets and desktops
            <BackgroundBlur>
                <div class="py-8 hidden sm:block">
                    // A select element that allows the user to add timezones to the carousel
                    <TimezoneSelect timezones_query set_timezones_query/>
                </div>
            </BackgroundBlur>

            // Drawer for mobiles
            <FloatingButton/>
        </TimezoneDrawer>

    }
}

#[component]
pub fn WelcomeText() -> impl IntoView {
    view! {
        <Introtext>
            <IntroTitle>"Compare timezones, fast"</IntroTitle>
            <IntroSubtitle>
                <ul id="sub-headings" class="list-disc">
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
          <label for="drawer-switch" class="drawer-button btn btn-lg btn-circle btn-neutral">
              <svg
                aria-label="Add Timezone"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="2"
                stroke="currentColor"
                class="size-6"
              >
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
              </svg>
          </label>
        </div>
    }
}

/// This component depends on a label within its `children` having the class
/// `drawer-button` and the `for` attribute containing "drawer". This class and for
/// attribute makes that button the button that opens this drawer.
#[component]
fn TimezoneDrawer(
    timezones_query: Memo<Option<String>>,
    set_timezones_query: SignalSetter<Option<String>>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="drawer drawer-end">
          <input id="drawer-switch" type="checkbox" class="drawer-toggle" />
          <div class="drawer-content">
              {children()}
          </div>
          <div class="drawer-side">

            <label for="drawer-switch" aria-label="close sidebar" class="drawer-overlay"></label>
            <TimezoneDrawerContent timezones_query set_timezones_query/>

          </div>
        </div>
    }
}
