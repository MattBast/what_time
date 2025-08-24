use crate::components::{IntroSubtitle, IntroTitle, Introtext};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Introtext>
            <IntroTitle>"Home"</IntroTitle>
            <IntroSubtitle>"Hello World!"</IntroSubtitle>
        </Introtext>
    }
}
