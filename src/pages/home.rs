use crate::components::{
    AddTimezoneButton, BackgroundBlur, InlineLi, IntroSubtitle, IntroTitle, Introtext, NowButton,
    TimezoneDrawerContent, DRAWER_SWITCH_ID,
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
                when=move || should_show_compare(timezones_query.get())
                fallback=|| view! {
                    <BackgroundBlur>
                        <div class="pt-24">
                            <WelcomeText/>
                        </div>
                    </BackgroundBlur>
                }
            >
                <Compare timezones_query time_query set_time_query/>
            </Show>

            <BackgroundBlur>
                <div class="flex flex-wrap justify-center gap-3 py-8">
                    // Open drawer to add a new timezone.
                    <AddTimezoneButton/>

                    {move || should_show_compare(timezones_query.get()).then(|| view! {
                        // Set the current time to the current time in the user's timezone.
                        <NowButton set_time_query/>
                    })}

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

/// Whether the compare view should render instead of the welcome screen.
pub(crate) fn should_show_compare(timezones_query: Option<String>) -> bool {
    !url_query_to_timezones(timezones_query.unwrap_or_default()).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_show_compare_when_zone_query_is_empty() {
        assert!(!should_show_compare(None));
        assert!(!should_show_compare(Some(String::new())));
    }

    #[test]
    fn test_should_show_compare_when_zone_query_has_timezone() {
        assert!(should_show_compare(Some("Europe__London".into())));
        assert!(should_show_compare(Some(
            "Europe__London,Europe__Paris".into()
        )));
    }

    #[test]
    fn test_should_show_compare_when_zone_query_only_has_invalid_segments() {
        assert!(!should_show_compare(Some("Not_A_Zone,Also_Invalid".into())));
    }

    #[test]
    fn test_should_show_compare_when_zone_query_has_valid_and_invalid_segments() {
        assert!(should_show_compare(Some("Bad_Zone,Europe__Dublin".into())));
    }
}
