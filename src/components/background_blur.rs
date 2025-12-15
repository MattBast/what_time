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
    view! {
        <div class="bg-radial from-white dark:from-black from-20%">
            {children()}
        </div>
    }
}
