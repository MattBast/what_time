use crate::components::{IntroSubtitle, IntroTitle, Introtext};
use leptos::prelude::*;

#[component]
pub fn Compare() -> impl IntoView {
    view! {
        <Introtext>
            <IntroTitle>"Compare Timezones"</IntroTitle>
            <IntroSubtitle>"Want to know the difference between two or more timezones? Add some timezones below to see the difference."</IntroSubtitle>
        </Introtext>
    }
}
