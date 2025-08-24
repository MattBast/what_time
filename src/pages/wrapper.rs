use crate::components::{DarkModeToggle, Logo};
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn Wrapper() -> impl IntoView {
    view! {
        <div class="font-sans flex flex-col justify-center min-h-screen px-4 sm:px-8 lg:px-12">

            <DarkModeToggle/>

            <Logo/>

            <Outlet/>

        </div>
    }
}
