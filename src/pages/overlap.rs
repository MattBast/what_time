use crate::components::{
    AddTimezoneButton, BackgroundBlur, IntroSubtitle, IntroTitle, Introtext, NowButton, ViewToggle,
    DrawerRouteMode,
};
use crate::pages::TimezoneDrawer;
use crate::timezone::{sort_cities, utc_to_local_timezone};
use crate::url_parse::{find_city_by_slug, route_cities_slugs, url_query_to_cities};
use crate::{CURRENT_TIME, ZONE};
use chrono::prelude::*;
use chrono_tz::Tz;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{query_signal, use_params_map};
use std::str::FromStr;

const EXTRA_CITIES: &str = "cities";

#[component]
pub fn Overlap(
    cities_query: Memo<Option<String>>,
    time_query: Memo<Option<i64>>,
    set_time_query: SignalSetter<Option<i64>>,
) -> impl IntoView {
    let (get_cities, set_cities) = signal(Vec::new());

    // Listen for the `zone` url query to change and when it does, re-render the cities.
    Effect::new(move || {
        let query = cities_query.get().unwrap_or_default();
        let mut cities = url_query_to_cities(query);
        sort_cities(&mut cities);
        set_cities.set(cities);
    });

    let cities_memo = Memo::new(move |_| get_cities.get());

    // Compute the overlaps for the 24 hours of the timeline
    let overlaps = Memo::new(move |_| {
        let cities = get_cities.get();
        if cities.is_empty() {
            return vec![false; 24];
        }

        let first_city = &cities[0];
        let first_city_tz = Tz::from_str(&first_city.timezone).unwrap_or(Tz::UTC);
        let base_dt = utc_to_local_timezone(time_query.get(), first_city_tz);

        let start_of_day = match base_dt.date_naive().and_hms_opt(0, 0, 0) {
            Some(naive_time) => match first_city_tz.from_local_datetime(&naive_time) {
                chrono::LocalResult::Single(dt) => dt,
                chrono::LocalResult::Ambiguous(dt1, _) => dt1,
                chrono::LocalResult::None => base_dt,
            },
            None => base_dt,
        };
        let start_of_day_utc = start_of_day.with_timezone(&Utc);

        (0..24)
            .map(|i| {
                let hour_utc = start_of_day_utc + chrono::Duration::hours(i);
                cities.iter().all(|city| {
                    let tz = Tz::from_str(&city.timezone).unwrap_or(Tz::UTC);
                    let local_dt = hour_utc.with_timezone(&tz);
                    let hr = local_dt.hour();
                    let wk = local_dt.weekday();

                    let is_working_hour = hr >= 9 && hr < 17;
                    let is_working_day = wk != Weekday::Sat && wk != Weekday::Sun;

                    is_working_hour && is_working_day
                })
            })
            .collect::<Vec<bool>>()
    });

    // Compute the index of the hour currently selected
    let selected_index = Memo::new(move |_| {
        let cities = get_cities.get();
        if cities.is_empty() {
            return None;
        }
        let first_city = &cities[0];
        let first_city_tz = Tz::from_str(&first_city.timezone).unwrap_or(Tz::UTC);
        let base_dt = utc_to_local_timezone(time_query.get(), first_city_tz);

        Some(base_dt.hour() as usize)
    });

    // Compute the start of day UTC reference timestamp
    let start_of_day_utc_ref = Memo::new(move |_| {
        let cities = get_cities.get();
        if cities.is_empty() {
            return Utc::now();
        }
        let first_city = &cities[0];
        let first_city_tz = Tz::from_str(&first_city.timezone).unwrap_or(Tz::UTC);
        let base_dt = utc_to_local_timezone(time_query.get(), first_city_tz);

        let start_of_day = match base_dt.date_naive().and_hms_opt(0, 0, 0) {
            Some(naive_time) => match first_city_tz.from_local_datetime(&naive_time) {
                chrono::LocalResult::Single(dt) => dt,
                chrono::LocalResult::Ambiguous(dt1, _) => dt1,
                chrono::LocalResult::None => base_dt,
            },
            None => base_dt,
        };
        start_of_day.with_timezone(&Utc)
    });

    view! {
        <BackgroundBlur>
            <div class="flex flex-col items-center w-full px-2 pt-4">
                <ViewToggle
                    is_compare=false
                    cities=cities_memo
                    time_query=time_query
                    cities_query=cities_query
                />

                // Timezone overlap timeline cards
                <div class="w-full max-w-7xl mx-auto border border-base-300 rounded-3xl bg-base-100/50 backdrop-blur-md p-6 shadow-xl overflow-hidden mb-6">
                    <div class="overflow-x-auto w-full no-scrollbar">
                        <div class="min-w-[900px] flex flex-col gap-6">
                            <For
                                each=move || get_cities.get()
                                key=|city| city.slug.clone()
                                children=move |city| {
                                    let city_clone = city.clone();
                                    let tz = Tz::from_str(&city.timezone).unwrap_or(Tz::UTC);
                                    let current_local_time = utc_to_local_timezone(time_query.get(), tz);
                                    let abbreviation = current_local_time.format("%Z").to_string();

                                    view! {
                                        <div class="flex items-center gap-6 py-4 border-b border-base-200 last:border-0">
                                            // City Metadata Card
                                            <div class="w-56 shrink-0 flex flex-col justify-center">
                                                <div class="font-bold text-base flex items-center gap-2">
                                                    <span>{city_clone.emoji.clone()}</span>
                                                    <span class="truncate">{city_clone.name.clone()}</span>
                                                </div>
                                                <div class="text-xs opacity-65 truncate mt-0.5">
                                                    {city_clone.country.clone()} " (" {abbreviation} ")"
                                                </div>
                                            </div>

                                            // Timeline track and items
                                            <div class="flex flex-1 justify-between items-center relative py-2 bg-base-200/30 rounded-2xl px-4 border border-base-200/50">
                                                // Center track line
                                                <div class="absolute left-6 right-6 h-[2px] bg-base-300 top-[28px] z-0"></div>

                                                // 24 Hour blocks
                                                <For
                                                    each=move || 0..24
                                                    key=|&i| i
                                                    children=move |i| {
                                                        let hour_utc = start_of_day_utc_ref.get() + chrono::Duration::hours(i as i64);
                                                        let local_dt = hour_utc.with_timezone(&tz);
                                                        let hour_val = local_dt.hour();
                                                        let wk = local_dt.weekday();

                                                        let is_working_hour = hour_val >= 9 && hour_val < 17;
                                                        let is_working_day = wk != Weekday::Sat && wk != Weekday::Sun;
                                                        let is_work = is_working_hour && is_working_day;
                                                        let is_overlap_hour = overlaps.get()[i];
                                                        let is_selected = selected_index.get() == Some(i);

                                                        let hour_label = format!(
                                                            "{}{}",
                                                            local_dt.format("%l"),
                                                            local_dt.format("%P").to_string().to_lowercase()
                                                        );
                                                        let weekday_label = local_dt.format("%a").to_string();

                                                        // Setup tooltips and active classes
                                                        let tooltip_text = format!(
                                                            "{} ({}) - {}",
                                                            local_dt.format("%I:%M %p"),
                                                            weekday_label,
                                                            if is_overlap_hour {
                                                                "Overlap Working Hour!"
                                                            } else if is_work {
                                                                "Working Hour"
                                                            } else {
                                                                "Non-Working Hour"
                                                            }
                                                        );

                                                        view! {
                                                            <div
                                                                class=move || {
                                                                    let base = "flex flex-col items-center flex-1 min-w-[32px] relative z-10 py-1 rounded-lg cursor-pointer transition-all duration-200 tooltip tooltip-top";
                                                                    if is_overlap_hour {
                                                                        format!("{base} bg-success/10 hover:bg-success/20 border border-success/15")
                                                                    } else {
                                                                        format!("{base} hover:bg-base-300/40")
                                                                    }
                                                                }
                                                                data-tip=tooltip_text
                                                                on:click=move |_| {
                                                                    set_time_query.set(Some(hour_utc.timestamp()));
                                                                }
                                                            >
                                                                // Hour text label
                                                                <span class=move || {
                                                                    let base = "text-[10px] font-bold select-none transition-colors";
                                                                    if is_overlap_hour {
                                                                        format!("{base} text-success-content")
                                                                    } else if is_work {
                                                                        format!("{base} text-info-content")
                                                                    } else {
                                                                        format!("{base} text-base-content/50")
                                                                    }
                                                                }>
                                                                    {hour_label}
                                                                </span>

                                                                // Dot marker
                                                                <div class=move || {
                                                                    let base = "w-3.5 h-3.5 rounded-full border-2 border-base-100 transition-all duration-200 mt-1 shadow-sm";
                                                                    if is_overlap_hour {
                                                                        format!("{base} bg-success scale-110 shadow-[0_0_8px_var(--color-success)]")
                                                                    } else if is_work {
                                                                        format!("{base} bg-info")
                                                                    } else {
                                                                        format!("{base} bg-base-300")
                                                                    }
                                                                }></div>

                                                                // Weekday label
                                                                <span class="text-[8px] opacity-60 mt-1 select-none font-medium">
                                                                    {weekday_label}
                                                                </span>
                                                            </div>
                                                        }
                                                    }
                                                />
                                            </div>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </div>
                </div>
            </div>
        </BackgroundBlur>
    }
}

#[component]
pub fn OverlapContent(
    timezones_query: Memo<Option<String>>,
    set_timezones_query: SignalSetter<Option<String>>,
    time_query: Memo<Option<i64>>,
    set_time_query: SignalSetter<Option<i64>>,
) -> impl IntoView {
    view! {
        <TimezoneDrawer timezones_query set_timezones_query route_mode=DrawerRouteMode::Overlap>
            <Show
                when=move || crate::pages::should_show_compare(timezones_query.get())
                fallback=|| view! {
                    <BackgroundBlur>
                        <div class="pt-24">
                            <OverlapWelcomeText/>
                        </div>
                    </BackgroundBlur>
                }
            >
                <Overlap cities_query=timezones_query time_query set_time_query/>
            </Show>

            <BackgroundBlur>
                <div class="flex flex-wrap justify-center gap-3 py-8">
                    <AddTimezoneButton/>

                    {move || crate::pages::should_show_compare(timezones_query.get()).then(|| view! {
                        <NowButton set_time_query/>
                    })}
                </div>
            </BackgroundBlur>
        </TimezoneDrawer>
    }
}

#[component]
pub fn OverlapWelcomeText() -> impl IntoView {
    view! {
        <Introtext>
            <IntroTitle>"Find overlap times, fast"</IntroTitle>
            <IntroSubtitle>
                <ul id="sub-headings" class="list-disc">
                    <crate::components::InlineLi>"🙂 Pick your city."</crate::components::InlineLi>
                    <crate::components::InlineLi>"😀 Compare working hour overlaps."</crate::components::InlineLi>
                    <crate::components::InlineLi>"😁 Find the perfect meeting slot."</crate::components::InlineLi>
                </ul>
            </IntroSubtitle>
        </Introtext>
    }
}

#[component]
pub fn OverlapCityPair() -> impl IntoView {
    let params = use_params_map();
    let (extra_cities_query, _) = query_signal::<String>(EXTRA_CITIES);
    let (zone_query, set_zone_query) = query_signal::<String>(ZONE);
    let (time_query, set_time_query) = query_signal::<i64>(CURRENT_TIME);

    let route_cities_query = Memo::new(move |_| {
        params.with(|params| route_cities_slugs(params.get("cities"), extra_cities_query.get()))
    });

    let cities_query = Memo::new(move |_| {
        let mut slugs = Vec::new();
        let rc = route_cities_query.get();
        if !rc.is_empty() {
            slugs.push(rc);
        }
        if let Some(zone) = zone_query.get() {
            slugs.push(zone);
        }
        let merged = slugs.join(",");

        (!merged.is_empty()).then_some(merged)
    });

    let page_title = Memo::new(move |_| {
        params.with(|params| {
            let cities_param = params.get("cities").unwrap_or_default();
            let slugs: Vec<&str> = cities_param.split('/').filter(|s| !s.is_empty()).collect();
            route_cities_page_title(&slugs)
        })
    });

    let (get_cities, set_cities) = signal(Vec::new());

    // Listen for the `zone` url query to change and when it does, re-render the cities.
    Effect::new(move || {
        let query = cities_query.get().unwrap_or_default();
        let mut cities = url_query_to_cities(query);
        sort_cities(&mut cities);
        set_cities.set(cities);
    });

    let cities_memo = Memo::new(move |_| get_cities.get());

    // Compute the overlaps for the 25 hours (0..=24) of the timeline
    let overlaps = Memo::new(move |_| {
        let cities = get_cities.get();
        if cities.is_empty() {
            return vec![false; 25];
        }

        let first_city = &cities[0];
        let first_city_tz = Tz::from_str(&first_city.timezone).unwrap_or(Tz::UTC);
        let base_dt = utc_to_local_timezone(time_query.get(), first_city_tz);

        let start_of_day = match base_dt.date_naive().and_hms_opt(0, 0, 0) {
            Some(naive_time) => match first_city_tz.from_local_datetime(&naive_time) {
                chrono::LocalResult::Single(dt) => dt,
                chrono::LocalResult::Ambiguous(dt1, _) => dt1,
                chrono::LocalResult::None => base_dt,
            },
            None => base_dt,
        };
        let start_of_day_utc = start_of_day.with_timezone(&Utc);

        (0..=24)
            .map(|i| {
                let hour_utc = start_of_day_utc + chrono::Duration::hours(i);
                cities.iter().all(|city| {
                    let tz = Tz::from_str(&city.timezone).unwrap_or(Tz::UTC);
                    let local_dt = hour_utc.with_timezone(&tz);
                    let hr = local_dt.hour();
                    let wk = local_dt.weekday();

                    let is_working_hour = hr >= 9 && hr < 17;
                    let is_working_day = wk != Weekday::Sat && wk != Weekday::Sun;

                    is_working_hour && is_working_day
                })
            })
            .collect::<Vec<bool>>()
    });

    // Compute the index of the hour currently selected
    let selected_index = Memo::new(move |_| {
        let cities = get_cities.get();
        if cities.is_empty() {
            return None;
        }
        let first_city = &cities[0];
        let first_city_tz = Tz::from_str(&first_city.timezone).unwrap_or(Tz::UTC);
        let base_dt = utc_to_local_timezone(time_query.get(), first_city_tz);

        Some(base_dt.hour() as usize)
    });

    // Compute the start of day UTC reference timestamp
    let start_of_day_utc_ref = Memo::new(move |_| {
        let cities = get_cities.get();
        if cities.is_empty() {
            return Utc::now();
        }
        let first_city = &cities[0];
        let first_city_tz = Tz::from_str(&first_city.timezone).unwrap_or(Tz::UTC);
        let base_dt = utc_to_local_timezone(time_query.get(), first_city_tz);

        let start_of_day = match base_dt.date_naive().and_hms_opt(0, 0, 0) {
            Some(naive_time) => match first_city_tz.from_local_datetime(&naive_time) {
                chrono::LocalResult::Single(dt) => dt,
                chrono::LocalResult::Ambiguous(dt1, _) => dt1,
                chrono::LocalResult::None => base_dt,
            },
            None => base_dt,
        };
        start_of_day.with_timezone(&Utc)
    });

    view! {
        <Title text=move || format!("Overlap: {}", page_title.get())/>
        <TimezoneDrawer timezones_query=cities_query set_timezones_query=set_zone_query route_mode=DrawerRouteMode::Overlap>
            <BackgroundBlur>
                <div class="flex flex-col items-center w-full px-2 pt-4">
                    <ViewToggle
                        is_compare=false
                        cities=cities_memo
                        time_query=time_query
                        cities_query=cities_query
                    />

                    <Show
                        when=move || !get_cities.get().is_empty()
                        fallback=|| view! {
                            <div class="text-center py-12 opacity-60">
                                "Please select at least one city/timezone to compare."
                            </div>
                        }
                    >
                        <div class="w-full max-w-3xl mx-auto border border-base-300 rounded-3xl bg-base-100/50 backdrop-blur-md p-6 shadow-xl overflow-hidden mb-6">
                            <div class="text-center mb-6">
                                <h2 class="text-lg font-bold tracking-tight">"Timezone Overlap Timeline"</h2>
                                <p class="text-xs opacity-60 mt-1 flex flex-col gap-1 items-center justify-center">
                                    <span>"Highlighting overlapping working hours (9:00 AM - 5:00 PM local time)"</span>
                                    <span class="flex items-center gap-1.5 mt-1">
                                        <span class="badge badge-success badge-xs"></span>
                                        "Overlap Hour"
                                    </span>
                                </p>
                            </div>

                            <div class="overflow-x-auto w-full no-scrollbar">
                                <ul class="timeline timeline-vertical mx-auto w-fit" style="grid-template-columns: 0px auto 1fr;">
                                    <For
                                        each=move || 0..=24
                                        key=|&i| i
                                        children=move |i| {
                                            let hour_utc = start_of_day_utc_ref.get() + chrono::Duration::hours(i as i64);

                                            let is_overlap_hour = overlaps.get()[i];

                                            view! {
                                                <li>
                                                    {move || {
                                                        if i > 0 {
                                                            let is_overlap_prev = overlaps.get()[i - 1] && overlaps.get()[i];
                                                            let class = if is_overlap_prev { "bg-success" } else { "" };
                                                            Some(view! { <hr class=class /> })
                                                        } else {
                                                            None
                                                        }
                                                    }}


                                                    {move || {
                                                        if is_overlap_hour {
                                                            view! {
                                                                <div
                                                                    class="timeline-middle bg-success w-4.5 h-4.5 rounded-full border-2 border-base-100 shadow-sm z-10 cursor-pointer"
                                                                    on:click=move |_| {
                                                                        set_time_query.set(Some(hour_utc.timestamp()));
                                                                    }
                                                                ></div>
                                                            }.into_any()
                                                        } else {
                                                            view! {
                                                                <div
                                                                    class="timeline-middle w-4 h-4 rounded-full bg-base-300 border-2 border-base-100 hover:bg-neutral hover:scale-110 transition-all duration-200 cursor-pointer"
                                                                    on:click=move |_| {
                                                                        set_time_query.set(Some(hour_utc.timestamp()));
                                                                    }
                                                                ></div>
                                                            }.into_any()
                                                        }
                                                    }}

                                                    <div class="timeline-end bg-base-200/20 hover:bg-base-200/50 transition-colors p-3 rounded-2xl border border-base-200/40 flex flex-col gap-2 min-w-[280px] ml-2 my-1">
                                                        {move || {
                                                            let cities_clone = get_cities.get();
                                                            cities_clone.into_iter().map(|city| {
                                                                let city_tz = Tz::from_str(&city.timezone).unwrap_or(Tz::UTC);
                                                                let local_dt = hour_utc.with_timezone(&city_tz);
                                                                let time_str = local_dt.format("%I:%M %p").to_string();
                                                                let day_str = local_dt.format("%a").to_string();

                                                                view! {
                                                                    <div class="flex items-center justify-between text-xs py-0.5">
                                                                        <div class="flex items-center gap-1.5 font-medium flex-1 min-w-0 pr-2">
                                                                            <span class="shrink-0">{city.emoji.clone()}</span>
                                                                            <span class="opacity-80 truncate">{city.name.clone()}</span>
                                                                        </div>
                                                                        <div class="flex items-center gap-2 font-mono shrink-0">
                                                                            <span class="opacity-70">{time_str}</span>
                                                                            <span class="text-[10px] opacity-45">{day_str}</span>
                                                                        </div>
                                                                    </div>
                                                                }
                                                            }).collect_view()
                                                        }}
                                                    </div>

                                                    {move || {
                                                        if i < 24 {
                                                            let is_overlap_next = overlaps.get()[i] && overlaps.get()[i + 1];
                                                            let class = if is_overlap_next { "bg-success" } else { "" };
                                                            Some(view! { <hr class=class /> })
                                                        } else {
                                                            None
                                                        }
                                                    }}
                                                </li>
                                            }
                                        }
                                    />
                                </ul>
                            </div>
                        </div>
                    </Show>
                </div>
            </BackgroundBlur>
            <BackgroundBlur>
                <div class="flex flex-wrap justify-center gap-3 py-8">
                    <AddTimezoneButton/>
                    <NowButton set_time_query/>
                </div>
            </BackgroundBlur>
        </TimezoneDrawer>
    }
}

#[component]
pub fn OverlapHome() -> impl IntoView {
    let (cities_query, set_cities_query) = query_signal::<String>(ZONE);
    let (time_query, set_time_query) = query_signal::<i64>(CURRENT_TIME);

    view! {
        <OverlapContent
            timezones_query=cities_query
            set_timezones_query=set_cities_query
            time_query
            set_time_query
        />
    }
}

fn route_cities_page_title(slugs: &[&str]) -> String {
    if slugs.is_empty() {
        "What Time".to_string()
    } else {
        let titles: Vec<String> = slugs
            .iter()
            .map(|slug| format!("{} Time", city_slug_to_title(slug)))
            .collect();
        format!("{} | What Time", titles.join(" vs "))
    }
}

fn city_slug_to_title(slug: &str) -> String {
    if let Some(city) = find_city_by_slug(slug) {
        city.name.clone()
    } else {
        slug.split(['-', '_'])
            .filter(|word| !word.is_empty())
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
