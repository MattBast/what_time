use crate::components::{DarkModeToggle, IntroSubtitle, IntroTitle, Introtext, Logo};
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Compare() -> impl IntoView {
    view! {
        <Title text="What Time"/>
        <main>
            <div class="font-sans flex flex-col justify-center min-h-screen px-4 sm:px-8 lg:px-12">

                <DarkModeToggle/>

                <Logo/>

                <Introtext>
                    <IntroTitle>"Compare Timezones"</IntroTitle>
                    <IntroSubtitle>"Want to know the difference between two or more timezones? Add some timezones below to see the difference."</IntroSubtitle>
                </Introtext>

                // This does nothing until the routes change to put a component in it.
                // <Outlet/>

            </div>
        </main>
    }
}
