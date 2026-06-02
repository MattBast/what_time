use crate::components::{
    AddTimezoneButton, BackgroundBlur, InlineLi, IntroSubtitle, IntroTitle, Introtext,
    TimePicker, TimezoneDrawerContent, DRAWER_SWITCH_ID,
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

            <BackgroundBlur>
                <div class="flex justify-center py-8">
                    <AddTimezoneButton/>
                </div>
            </BackgroundBlur>
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

/// Wraps page content and the timezone drawer. A control with class `drawer-button` and
/// `for` pointing at the drawer checkbox opens the drawer.
#[component]
fn TimezoneDrawer(
    timezones_query: Memo<Option<String>>,
    set_timezones_query: SignalSetter<Option<String>>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="drawer drawer-end">
          <input id=DRAWER_SWITCH_ID type="checkbox" class="drawer-toggle" />
          <div class="drawer-content">
              {children()}
          </div>
          <div class="drawer-side">
            <label for=DRAWER_SWITCH_ID aria-label="close sidebar" class="drawer-overlay"></label>
            <TimezoneDrawerContent timezones_query set_timezones_query/>
          </div>
        </div>
    }
}
