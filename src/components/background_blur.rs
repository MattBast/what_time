use leptos::html::div;
use leptos::prelude::*;

/// Add a solid colour blur behind an element.
/// Use it like this
/// ```html
/// <BackgroundBlur>
///    <Child/>
/// </BackgroundBlur>
/// ```
#[component]
pub fn BackgroundBlur(children: Children) -> impl IntoView {
    div()
        .attr("class", "bg-radial from-base-100 from-20%")
        .child(children())
}
