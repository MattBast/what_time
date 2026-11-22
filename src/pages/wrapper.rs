use crate::components::{logo, theme_toggle};
use leptos::html::div;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn Wrapper() -> impl IntoView {
    view! {
        <div
            class=concat!(
                // Layout
                "flex flex-col justify-center min-h-screen ",
                "px-4 sm:px-8 lg:px-12 ",

                // Base background
                "bg-base-100 ",

                // Dot pattern
                "bg-size-[16px_16px] ",
                "bg-[radial-gradient(var(--color-base-300)_1px,transparent_1px)]"
            )
        >

            <Header/>

            <Outlet/>

        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    div()
        .attr("class", "fixed top-5 left-0 right-0 z-50")
        .child(
            div().attr("class", "mx-auto max-w-7xl px-4 sm:px-8").child(
                div()
                    .attr("class", "flex items-center justify-between")
                    .child((logo(), theme_toggle())),
            ),
        )
}
