use leptos::prelude::*;

/// Allow the user to toggle between light and dark mode
#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <div class="flex flex-1 justify-end md:justify-center">
            <div
                class="pointer-events-auto md:hidden"
                data-headlessui-state=""
            >
                <button
                    class="group flex items-center rounded-full bg-white/90 px-4 py-2 text-sm font-medium text-zinc-800 shadow-lg ring-1 shadow-zinc-800/5 ring-zinc-900/5 backdrop-blur-sm dark:bg-zinc-800/90 dark:text-zinc-200 dark:ring-white/10 dark:hover:ring-white/20"
                    type="button"
                    aria-expanded="false"
                    data-headlessui-state=""
                    id="headlessui-popover-button-_R_3dkmlb_"
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
            </div>

            <nav class="pointer-events-auto hidden md:block">

                <ul
                    class="flex rounded-full bg-white/90 px-3 text-sm font-medium text-zinc-800 shadow-lg ring-1 shadow-zinc-800/5 ring-zinc-900/5 backdrop-blur-sm dark:bg-zinc-800/90 dark:text-zinc-200 dark:ring-white/10"
                >

                    <li>
                        <a
                            class="relative block px-3 py-2 transition hover:text-teal-500 dark:hover:text-teal-400"
                            href="/"
                        >
                            "Home"
                        </a>
                    </li>

                    <li>
                        <a
                            class="relative block px-3 py-2 transition hover:text-teal-500 dark:hover:text-teal-400"
                            href="/compare"
                        >
                            "Compare"
                        </a>
                    </li>

                    <li>
                        <a
                            class="relative block px-3 py-2 transition hover:text-teal-500 dark:hover:text-teal-400"
                            href="/carousel"
                        >
                            "Carousel"
                        </a>
                    </li>

                </ul>
            </nav>

        </div>
    }
}
