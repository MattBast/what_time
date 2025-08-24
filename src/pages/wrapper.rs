use crate::components::{DarkModeToggle, Logo, Nav};
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn Wrapper() -> impl IntoView {
    view! {
        <div class="font-sans flex flex-col justify-center min-h-screen px-4 sm:px-8 lg:px-12">

            <div class="fixed top-5 left-0 right-0">
                <div class="mx-auto w-full max-w-7xl lg:px-8">
                    <div class="relative px-4 sm:px-8 lg:px-12">
                        <div class="mx-auto max-w-2xl lg:max-w-5xl">
                            <div class="relative flex gap-4">
                                <div class="flex flex-1">
                                    // for the Logo
                                </div>
                                <div class="flex flex-1 justify-end md:justify-center">
                                    <Nav/>
                                </div>
                                <div class="flex justify-end md:flex-1">
                                    <DarkModeToggle/>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <Logo/>

            <Outlet/>

        </div>
    }
}
