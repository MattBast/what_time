use crate::components::{InlineLi, IntroSubtitle, IntroTitle, Introtext};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
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
