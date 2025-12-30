use crate::components::{DarkModeToggle, Logo};
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn Wrapper() -> impl IntoView {
    view! {
        <div
            class=concat!(
                // Layout
                "font-sans flex flex-col justify-center min-h-screen ",
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
    view! {
        <div class="fixed top-5 left-0 right-0">
            <div class="mx-auto w-full max-w-7xl">
                <div class="relative px-4 sm:px-8">
                        <div class="relative flex gap-4">

                            <div class="flex flex-1">
                                <Logo/>
                            </div>

                            <div class="flex justify-end md:flex-1">
                                <DarkModeToggle/>
                            </div>

                        </div>
                </div>
            </div>
        </div>
    }
}
