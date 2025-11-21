use crate::components::{DarkModeToggle, Logo};
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn Wrapper() -> impl IntoView {
    view! {
        <div class=format!(
            "{} {} {} {}",
            "font-sans flex flex-col justify-center min-h-screen px-4 sm:px-8 lg:px-12",
            "bg-size-[16px_16px]", // repeating dot background
            // "bg-[radial-gradient(#f4f4f5_1px,transparent_1px)]", // zinc-100
            "bg-[radial-gradient(#e4e4e7_1px,transparent_1px)]", // zinc-200
            "dark:bg-[radial-gradient(#18181b_1px,transparent_1px)]", // zinc-800
        )
        >

            // Top-right radial
            <div class="pointer-events-none fixed top-0 right-0 w-[40rem] h-[40rem] blur-3xl
                        bg-[radial-gradient(circle_at_top_right,theme(colors.teal.50),transparent_70%)]
                        dark:bg-none
            ">
            </div>

            // Bottom-left radial
            <div class="pointer-events-none fixed bottom-0 left-0 w-[40rem] h-[40rem] blur-3xl
                        bg-[radial-gradient(circle_at_bottom_left,theme(colors.teal.50),transparent_70%)]
                        dark:bg-none
            ">
            </div>

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
                                // {!url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty().then(|| view! {
                                    <Logo small=true/>
                                // })}
                            </div>
                            // <div class="flex flex-1 justify-end md:justify-center">
                            //     <Nav/>
                            // </div>
                            <div class="flex justify-end md:flex-1">
                                <DarkModeToggle/>
                            </div>
                        </div>
                </div>
            </div>
        </div>
    }
}
