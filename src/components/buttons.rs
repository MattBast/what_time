use leptos::prelude::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button class="btn btn-outline bg-base-100">
            {children()}
        </button>
    }
}
