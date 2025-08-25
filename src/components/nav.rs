use leptos::prelude::*;
use leptos_router::hooks::use_location;

/// Allow the user to toggle between light and dark mode
#[component]
pub fn Nav() -> impl IntoView {
    let (show_dropdown, set_show_dropdown) = signal(false);

    view! {
        // Small screen menu
        <div
            class="pointer-events-auto md:hidden"
            data-headlessui-state=""
        >
            // Button to open dropdown navigation menu
            <button
                on:click=move |_| set_show_dropdown.set(true)
                class="group flex items-center rounded-full bg-white/90 px-4 py-2 text-sm font-medium text-zinc-800 shadow-lg ring-1 shadow-zinc-800/5 ring-zinc-900/5 backdrop-blur-sm dark:bg-zinc-800/90 dark:text-zinc-200 dark:ring-white/10 dark:hover:ring-white/20"
                type="button"
                aria-expanded="false"
                data-headlessui-state=""
            >
                "Menu"
                <svg
                    viewBox="0 0 8 6"
                    aria-hidden="true"
                    class="ml-3 h-auto w-2 stroke-zinc-500 group-hover:stroke-zinc-700 dark:group-hover:stroke-zinc-400"
                >
                    <path
                        d="M1.75 1.75 4 4.25l2.25-2.5"
                        fill="none"
                        stroke-width="1.5"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                    </path>
                </svg>
            </button>

            // Blurred cover for page when menu is open
            <BlurredScreen show_screen=show_dropdown/>

            // Popup navigation menu
            <div
                class="fixed inset-x-4 top-8 z-51 origin-top rounded-3xl bg-white p-8 ring-1 ring-zinc-900/5 duration-150 data-closed:scale-95 data-closed:opacity-0 data-enter:ease-out data-leave:ease-in dark:bg-zinc-900 dark:ring-zinc-800"
                class=("hidden", move || !show_dropdown.get())
                tabindex="-1"
                data-headlessui-state="open"
                data-open=""
                style="--button-width: 88.671875px;"
            >
                <div class="flex flex-row-reverse items-center justify-between">
                    // Button to close the dropdown menu
                    <button
                        on:click=move |_| set_show_dropdown.set(false)
                        aria-label="Close menu"
                        class="-m-1 p-1"
                        type="button"
                        data-headlessui-state="open active"
                        data-open=""
                        data-active=""
                    >
                        <svg
                            viewBox="0 0 24 24"
                            aria-hidden="true"
                            class="h-6 w-6 text-zinc-500 dark:text-zinc-400"
                        >
                            <path
                                d="m17.25 6.75-10.5 10.5M6.75 6.75l10.5 10.5"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="1.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                            </path>
                        </svg>
                    </button>

                    <h2 class="text-sm font-medium text-zinc-600 dark:text-zinc-400">"Navigation"</h2>
                </div>

                <nav class="mt-6">
                    <ul class="-my-2 divide-y divide-zinc-100 text-base text-zinc-800 dark:divide-zinc-100/5 dark:text-zinc-300">
                        <SmallNavLink href="/".into() set_show_dropdown>"Home"</SmallNavLink>
                        <SmallNavLink href="/compare".into() set_show_dropdown>"Compare"</SmallNavLink>
                        <SmallNavLink href="/carousel".into() set_show_dropdown>"Carousel"</SmallNavLink>
                    </ul>
                </nav>
            </div>
        </div>

        // Large screen menu
        <nav class="pointer-events-auto hidden md:block">

            <ul
                class="flex rounded-full bg-white/90 px-3 text-sm font-medium text-zinc-800 shadow-lg ring-1 shadow-zinc-800/5 ring-zinc-900/5 backdrop-blur-sm dark:bg-zinc-800/90 dark:text-zinc-200 dark:ring-white/10"
            >

            <LargeNavLink href="/".into()>"Home"</LargeNavLink>
            <LargeNavLink href="/compare".into()>"Compare"</LargeNavLink>
            <LargeNavLink href="/carousel".into()>"Carousel"</LargeNavLink>

            </ul>
        </nav>
    }
}

#[component]
pub fn LargeNavLink(href: String, children: Children) -> impl IntoView {
    // Watch url query parameters so they can be included in the
    // href and not lost when the user clicks this link.
    let location = use_location();

    view! {
        <li>
            <a
                class="relative block px-3 py-2 transition hover:text-teal-500 dark:hover:text-teal-400"
                href=move || format!("{}?{}", href, location.search.get())
            >
                {children()}
            </a>
        </li>
    }
}

#[component]
pub fn SmallNavLink(
    href: String,
    set_show_dropdown: WriteSignal<bool>,
    children: Children,
) -> impl IntoView {
    // Watch url query parameters so they can be included in the
    // href and not lost when the user clicks this link.
    let location = use_location();

    view! {
        <li>
            <a
                class="block py-2"
                on:click=move |_| set_show_dropdown.set(false)
                data-headlessui-state="open active"
                data-open=""
                data-active=""
                href=move || format!("{}?{}", href, location.search.get())
            >
                {children()}
            </a>
        </li>
    }
}

/// A screen that when activated blurs the screen into the background so a popup can be placed on top.
#[component]
pub fn BlurredScreen(show_screen: ReadSignal<bool>) -> impl IntoView {
    view! {
        // Blurred cover for page when menu is open
        <div
            class="fixed inset-0 z-51 bg-zinc-800/40 backdrop-blur-xs duration-150 data-closed:opacity-0 data-enter:ease-out data-leave:ease-in dark:bg-black/80"
            class=("hidden", move || !show_screen.get())
            aria-hidden="true"
            data-headlessui-state="open"
            data-open=""
            style=""
        >
        </div>
    }
}
