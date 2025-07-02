use crate::timezone::{
    add_24_future_increments, add_24_past_increments, tz_to_city, tz_to_emoji, TimeIncrement,
};
use crate::timezone_select::TimezoneSelectTwo;
use crate::url_parse::{remove_timezone, url_query_to_time_increments};
use chrono_tz::{Tz, TZ_VARIANTS};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    hooks::query_signal,
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

#[component]
fn Carousel() -> impl IntoView {
    // Get a list of all the available timezones.
    let (tz_variants, set_tz_variants) =
        ArcRwSignal::new(TZ_VARIANTS.iter().map(|tz| tz.clone()).collect()).split();

    let timezones = ArcRwSignal::new(vec![
        // ArcRwSignal::new(vec![TimeIncrement::now(Tz::Europe__London)]),
        // ArcRwSignal::new(vec![TimeIncrement::now(Tz::Europe__Paris)]),
        // ArcRwSignal::new(vec![TimeIncrement::now(Tz::Asia__Kolkata)]),
        // ArcRwSignal::new(vec![TimeIncrement::now(Tz::Asia__Kathmandu)]),
        // ArcRwSignal::new(vec![TimeIncrement::now(Tz::America__New_York)]),
        // ArcRwSignal::new(vec![TimeIncrement::now(Tz::Asia__Tokyo)]),
    ]);

    let (get_timezones, set_timezones) = timezones.split();
    let set_timezones_2 = set_timezones.clone();

    let set_tz_variants_clone = set_tz_variants.clone();

    // Listen for the `zone` url wuery to change and when it does, re-render the timezones.
    let (url_query, set_url_query) = query_signal::<String>("zone");
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = url_query.get().unwrap_or_default();

        // Add the timezones from url to the carousel.
        timezones.set(url_query_to_time_increments(query.clone()));

        // Remove the timezones in the url from the dropdown options.
        set_tz_variants_clone.update(|variants| remove_timezone(query, variants));
    });

    // Some reminder code to add increments to the url parameters
    // let (q, set_q) = query_signal::<i32>("past_increments");
    // let (q, set_q) = query_signal::<i32>("future_increments");

    view! {

        <p class="text-white">{move || url_query.get().unwrap_or_default()}</p>

        <div class="flex justify-between mt-16 sm:mt-20 w-full">

            // When clicked, this button adds 24 past future time increments to all timezones
            <SideButton
                on:click=move |_| {

                    set_timezones.update(|tz_list| {
                        for timezone in tz_list {
                            timezone.update(|i| {
                                add_24_past_increments(i);
                            });
                        }
                    });

                }
            >
                "+ 24 Hours"
            </SideButton>

            // Creates a card for every time increment present.
            <div class="overflow-scroll mx-8 sm:mx-10 snap-x snap-mandatory scroll-smooth">

                <For
                    each=move || get_timezones.get()
                    key=|timezone| timezone.get_untracked().first().unwrap().timezone.clone()
                    children=move|increments| {

                        view! {
                            <div class="flex gap-5 py-4 sm:gap-8">

                                <For
                                    each=move || increments.get()
                                    key=|increment| increment.datetime.clone()
                                    let(hour)
                                >

                                    <Timecard hour/>

                                </For>


                            </div>
                        }
                    }
                />

            </div>

            // When clicked, this button adds 24 future future time increments to all timezones
            <SideButton
                on:click=move |_| {

                    set_timezones_2.update(|tz_list| {
                        for timezone in tz_list {
                            timezone.update(|i| {
                                add_24_future_increments(i);
                            });
                        }
                    });

                }
            >
                "+ 24 Hours"
            </SideButton>

        </div>

        // A select element that allows the user to add timezones to the carousel
        <TimezoneSelectTwo url_query set_url_query tz_variants set_tz_variants/>
    }
}

#[component]
fn Timecard(hour: TimeIncrement) -> impl IntoView {
    let now = hour.now.clone();
    let time = ArcRwSignal::new(hour);

    view! {
        <div class="rounded-2xl border border-zinc-100 dark:border-zinc-700/40 w-40 relative snap-center">
            // add a pinging dot if this is the current time
            {now.then(||view! {
                <span class="absolute top-2 right-2 flex size-3">
                    <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-teal-400 opacity-75"></span>
                    <span class="relative inline-flex size-3 rounded-full bg-teal-500"></span>
                </span>
            })}

            <div class="p-6 w-40 flex-none">
                <p class="my-2 text-sm text-zinc-600 dark:text-zinc-400">
                    {time.get_untracked().display_header()}
                </p>
                <h2 class="text-2xl font-semibold text-zinc-900 dark:text-zinc-100">
                    {time.get_untracked().display_time()}
                </h2>
                <p class="text-sm text-zinc-600 dark:text-zinc-400">
                    {time.get_untracked().display_date()}
                </p>
            </div>
        </div>
    }
}

#[component]
fn TimezoneSelect(
    url_query: Memo<Option<String>>,
    set_url_query: SignalSetter<Option<String>>,
    tz_variants: ArcReadSignal<Vec<Tz>>,
) -> impl IntoView {
    // 🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨
    // This select needs to be searchable because it has 600+ variants showing in it.
    // Maybe datalist could help. Or just use Thaws combobox element.
    // 🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨

    view! {

        <select
            on:change:target=move |ev| {
                let current_timezones = url_query.get_untracked().unwrap_or_default();
                let new_timezone = ev.target().value();
                if current_timezones.is_empty() {
                    set_url_query.set(Some(new_timezone));
                }
                else {
                    set_url_query.set(Some(current_timezones + "," + &new_timezone));
                }

            }
            // prop:value=move || value.get().to_string()
        >

            // The default option. Clicking this does nothing.
            <option value="">Select a timezone...</option>

            // Add an option per timezone not already showing
            <For
                each=move || tz_variants.get()
                key=|tz| tz.to_string().clone()
                children=move|tz| {

                    let tz_string = tz.to_string();
                    let url_value = tz_string.replace("/", "__");
                    let display_name = format!("{} {}", tz_to_emoji(&tz), tz_to_city(&tz));

                    view! {
                        <option value={url_value}>{display_name}</option>
                    }
                }
            />
        </select>
    }
}

/// Formats a timezone string like "America/New_York" into a more readable "New York"
/// or "Europe/London" into "London"
fn format_timezone_name(tz_str: &str) -> String {
    if let Some((_region, city)) = tz_str.split_once('/') {
        city.replace('_', " ")
    } else {
        tz_str.replace('_', " ")
    }
}

/// Helper to get all timezones with formatted names
fn get_all_timezones() -> Vec<(String, String, String)> {
    chrono_tz::TZ_VARIANTS
        .iter()
        .map(|tz| {
            let tz_string = tz.to_string();
            let url_value = tz_string.replace("/", "__");
            let display_name = format_timezone_name(&tz_string);
            (tz_string, url_value, display_name)
        })
        .collect()
}

fn filter_timezones(search: String) -> Vec<(String, String, String)> {
    // If nothing is typed into the search field, present all timezones
    if search.is_empty() {
        get_all_timezones()
    }
    // If something is typed into the search field, filter it to timezones
    // that contains the substring typed into the search field.
    else {
        get_all_timezones()
            .into_iter()
            .filter(|(tz_string, _, display_name)| {
                tz_string.to_lowercase().contains(&search)
                    || display_name.to_lowercase().contains(&search)
            })
            .collect()
    }
}

// #[component]
// fn Button(children: Children) -> impl IntoView {
//     view! {
//         <button class="cursor-pointer inline-flex items-center gap-2 justify-center rounded-md py-2 px-3 text-sm outline-offset-2 transition active:transition-none bg-zinc-800 font-semibold text-zinc-100 hover:bg-zinc-700 active:bg-zinc-800 active:text-zinc-100/70 dark:bg-zinc-700 dark:hover:bg-zinc-600 dark:active:bg-zinc-700 dark:active:text-zinc-100/70 ml-4 flex-none">
//             {children()}
//         </button>
//     }
// }

#[component]
fn SideButton(children: Children) -> impl IntoView {
    view! {
        <button
            class="cursor-pointer inline-flex items-center gap-2 justify-center rounded-md py-2 px-6 text-sm outline-offset-2 transition active:transition-none bg-zinc-50 font-medium text-zinc-900 hover:bg-zinc-100 active:bg-zinc-100 active:text-zinc-900/60 dark:bg-zinc-800/50 dark:text-zinc-300 dark:hover:bg-zinc-800 dark:hover:text-zinc-50 dark:active:bg-zinc-800/50 dark:active:text-zinc-50/70 group mt-6"
            style="writing-mode: vertical-rl"
        >
            {children()}
        </button>
    }
}
