use crate::carousel::Carousel;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/leptos_tailwind.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                <MetaTags/>
            </head>
            <body class="bg-zinc-50 dark:bg-black">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Title text="What Time"/>
            <main>
                <Routes fallback=|| "Page not found.">
                    <ParentRoute path=path!("/") view=Home>
                        <Route path=path!("") view=|| view! {
                            ""
                        }/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="font-sans flex flex-col justify-center min-h-screen px-4 sm:px-8 lg:px-12">

            <Introtext/>

            <Carousel/>

            // This does nothing until the routes change to put a component in it.
            <Outlet/>

        </div>
    }
}

#[component]
fn Introtext() -> impl IntoView {
    view! {
        <div class="mt-9">
            <div class="w-full max-w-7xl">
                <div class="relative">
                    <h1 class="text-4xl font-bold tracking-tight text-zinc-800 sm:text-5xl dark:text-zinc-100">
                        "Time Carousel"
                    </h1>

                    <p class="mt-6 text-base text-zinc-600 dark:text-zinc-400">
                        "Want to know the difference between two or more timezones? Add some timezones below to compare them hour by hour."
                    </p>
                </div>
            </div>
        </div>
    }
}
