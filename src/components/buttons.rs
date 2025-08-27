use leptos::html::Div;
use leptos::prelude::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button class="cursor-pointer inline-flex items-center gap-2 justify-center rounded-md py-2 px-3 text-sm outline-offset-2 transition active:transition-none bg-zinc-800 font-semibold text-zinc-100 hover:bg-zinc-700 active:bg-zinc-800 active:text-zinc-100/70 dark:bg-zinc-700 dark:hover:bg-zinc-600 dark:active:bg-zinc-700 dark:active:text-zinc-100/70 ml-4 flex-none">
            {children()}
        </button>
    }
}

#[component]
pub fn SideButton(node_ref: NodeRef<Div>) -> impl IntoView {
    view! {
        <div
            class="inline-flex items-center rounded-md px-4 py-2 text-sm leading-6 font-semibold transition duration-150 ease-in-out"
            node_ref=node_ref
        >
            <svg class="size-7 animate-spin text-zinc-600" viewBox="0 0 24 24">
                <circle class="opacity-25 fill-none" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
        </div>

    }
}
