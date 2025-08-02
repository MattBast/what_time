use crate::timezone::{tz_display, tz_to_city, tz_to_country, tz_to_emoji};
use crate::url_parse::remove_timezone;
use crate::url_parse::url_query_to_timezones;
use chrono_tz::{Tz, TZ_VARIANTS};
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::query_signal;

#[component]
pub fn TimezoneSelect() -> impl IntoView {
    // Watch the url.
    let (url_query, set_url_query) = query_signal::<String>("zone");

    // Get a list of all the available timezones to present in the dropdown.
    let (tz_variants, set_tz_variants) =
        ArcRwSignal::new(TZ_VARIANTS.iter().map(|tz| tz.clone()).collect()).split();
    let set_tz_variants_clone = set_tz_variants.clone();

    // A list of the timezones that have been selected.
    let (selected_tz_variants, set_selected_tz_variants) = ArcRwSignal::new(Vec::new()).split();

    // Get or set the value typed into the search input field.
    let (search_term, set_search_term) = signal(String::new());

    // Use this variable to decide if the dropdoiwn should be showing or not.
    let (show_dropdown, set_show_dropdown) = signal(false);

    // Watch the url query to decide which timezones to present in the dropdown.
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = url_query.get().unwrap_or_default();

        // Remove the timezones in the url from the dropdown options.
        set_tz_variants_clone.update(|variants| remove_timezone(query.clone(), variants));

        // And add them to the selected timezones list
        set_selected_tz_variants.set(url_query_to_timezones(query));
    });

    // Listen for the `search_term` to be changed
    Effect::new(move || {
        // Get the latest value typed into the select input field.
        // Trigger filter functionality whenever it changes.
        let search = search_term.get().to_lowercase();

        // Get all the timezones present in the url query
        let query = url_query.get().unwrap_or_default();
        let timezones_from_url = url_query_to_timezones(query);

        // Filter the timezones to include only the ones who contain
        // the substring typed into the search input field. Also don't
        // present timezones that are present in the url.
        if !search.is_empty() {
            let filtered: Vec<Tz> = TZ_VARIANTS
                .iter()
                .filter(|tz| {
                    let (_emoji, city, country) = tz_display(tz);

                    format!("{} {}", city.to_lowercase(), country.to_lowercase()).contains(&search)
                        && !timezones_from_url.contains(tz)
                })
                .copied()
                .collect();
            set_tz_variants.set(filtered);
        }
        // If nothing is typed into the search field, present all timezones
        // that are not already in the url query.
        else {
            let filtered: Vec<Tz> = TZ_VARIANTS
                .iter()
                .filter(|tz| !timezones_from_url.contains(tz))
                .copied()
                .collect();
            set_tz_variants.set(filtered);
        }
    });

    view! {
        <div class="relative mt-8 w-full">
            // A search input where the user can type and search for a timezone.
            <input
                type="text"
                placeholder="Search and add timezones..."
                class="w-full px-4 py-2 border border-zinc-300 dark:border-zinc-600 rounded-md bg-white dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100 focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-teal-500"
                prop:value=search_term
                // When the contents of the input is changed, update the `search_term`.
                // This tells the component to update the list if timezones in the
                // dropdown.
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    set_search_term.set(value);
                    set_show_dropdown.set(true);
                }
                on:focus=move |_| set_show_dropdown.set(true)
                // on:blur=move |_| set_show_dropdown.set(false)
                on:click=move |_| set_show_dropdown.set(true)
            />

            // Dropdown containing either all the timezones or a filtered subset of the timezones.
            <div
                class="absolute z-50 w-full mt-1 bg-white dark:bg-zinc-800 border border-zinc-300 dark:border-zinc-600 rounded-md shadow-lg max-h-60 overflow-y-auto"
                class:hidden=move || !show_dropdown.get()
            >
                <For
                    each=move || selected_tz_variants.get()
                    key=|tz| tz.to_string().clone()
                    children=move|tz| {

                        let tz_string = tz.to_string();
                        let tz_country = tz_to_country(&tz);
                        let url_value = tz_string.replace("/", "__");
                        let emoji = tz_to_emoji(&tz);
                        let city = tz_to_city(&tz);

                        view! {
                            <TimezoneSelectOption
                                emoji
                                city
                                tz_country
                                selected=true
                                // When clicked, the timezone is removed from the url query.
                                // There is logic elsewhere in the app to listen to the
                                // query and remove the timezone from the carousel.
                                on:click=move |_| {
                                    let mut current_timezones = url_query.get_untracked().unwrap_or_default();
                                    current_timezones = current_timezones.replace(&url_value, "");

                                    set_url_query.set(Some(current_timezones));

                                    // Empty the search term and hide the dropdown.
                                    set_search_term.set(String::new());
                                    set_show_dropdown.set(false);
                                }
                            />
                        }
                    }
                />


                // Add an option per timezone not already showing
                <For
                    each=move || tz_variants.get()
                    key=|tz| tz.to_string().clone()
                    children=move|tz| {

                        let tz_string = tz.to_string();
                        let tz_country = tz_to_country(&tz);
                        let url_value = tz_string.replace("/", "__");
                        let emoji = tz_to_emoji(&tz);
                        let city = tz_to_city(&tz);

                        view! {
                            <TimezoneSelectOption
                                emoji
                                city
                                tz_country
                                selected=false
                                // When clicked, the timezone is added to the url query.
                                // There is logic elsewhere in the app to listen to the
                                // query and update the carousel with the added timezone.
                                on:click=move |_| {
                                    let current_timezones = url_query.get_untracked().unwrap_or_default();
                                    if current_timezones.is_empty() {
                                        set_url_query.set(Some(url_value.clone()));
                                    } else {
                                        set_url_query.set(Some(current_timezones + "," + &url_value));
                                    }

                                    // Empty the search term and hide the dropdown.
                                    set_search_term.set(String::new());
                                    set_show_dropdown.set(false);
                                }
                            />
                        }
                    }
                />

            </div>
        </div>

        // Click outside to close dropdown
        <Show when=move || show_dropdown.get()>
            <div
                class="fixed inset-0 z-40"
                on:click=move |_| set_show_dropdown.set(false)
            ></div>
        </Show>
    }
}

#[component]
pub fn TimezoneSelectOption(
    emoji: String,
    city: String,
    tz_country: String,
    selected: bool,
) -> impl IntoView {
    view! {
        <div
            class="group w-full text-left px-4 py-2 border-none cursor-pointer text-zinc-900 dark:text-zinc-100"
            class=(["bg-teal-100", "dark:bg-teal-700", "hover:bg-red-100", "dark:hover:bg-red-700"], selected)
            class=(["bg-transparent", "hover:bg-zinc-100", "dark:hover:bg-zinc-700"], !selected)
        >
            <div class="flex justify-between items-center">
                <div>
                    <span class="font-medium">{emoji}</span>
                    <span class="font-medium">" "</span>
                    <span class="font-medium">{city}</span>
                </div>

                {if selected {
                    view! {
                        <span
                            class="
                                text-2xl font-bold text-teal-500 dark:text-teal-400
                                group-hover:text-red-500 group-hover:dark:text-red-400
                            "
                        >
                            <span class="group-hover:hidden">
                                <Icon icon=icondata::BiCheckRegular />
                            </span>
                            <span class="hidden group-hover:inline">
                                <Icon icon=icondata::BiTrashRegular />
                            </span>
                        </span>
                    }.into_any()
                } else {
                    view! {
                        <span class="text-sm text-zinc-500 dark:text-zinc-400">
                            {tz_country}
                        </span>
                    }.into_any()
                }}
            </div>
        </div>
    }
}
