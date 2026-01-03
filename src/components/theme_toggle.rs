use capitalize::Capitalize;
use leptos::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[component]
pub fn DarkModeToggle() -> impl IntoView {
    view! {
        <div class="dropdown mb-72">
          <div tabindex="0" role="button" class="btn">
            "Theme"

            <svg // <- Dropdown arrow
              width="12px"
              height="12px"
              class="inline-block h-2 w-2 fill-current opacity-60"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 2048 2048">
              <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
            </svg>
          </div>

          <ul tabindex="-1" class="dropdown-content overflow-scroll max-h-80 bg-base-300 rounded-box z-1 w-52 p-2 shadow-2xl">

            {
                Theme::iter()
                    .map(|theme| view! {
                        <li>
                          <input
                            type="radio"
                            name="theme-dropdown"
                            class="theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"
                            aria-label={capitalize(&theme)}
                            value={lowercase(&theme)} />
                        </li>
                    })
                    .collect::<Vec<_>>()
            }

          </ul>
        </div>
    }
}

#[derive(EnumIter, strum_macros::Display, Debug, PartialEq)]
enum Theme {
    Default,
    Light,
    Dark,
    Cupcake,
    Bumblebee,
    Emerald,
    Corporate,
    Synthwave,
    Retro,
    Cyberpunk,
    Valentine,
    Halloween,
    Garden,
    Forest,
    Aqua,
    Lofi,
    Pastel,
    Fantasy,
    Wireframe,
    Black,
    Luxury,
    Dracula,
    Cmyk,
    Autumn,
    Business,
    Acid,
    Lemonade,
    Night,
    Coffee,
    Winter,
    Dim,
    Nord,
    Sunset,
    Caramellatte,
    Abyss,
    Silk,
}

fn capitalize(theme: &Theme) -> String {
    theme.to_string().capitalize_first_only()
}

fn lowercase(theme: &Theme) -> String {
    theme.to_string().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_can_be_capitalized() {
        assert_eq!(capitalize(&Theme::Default), "Default");
        assert_eq!(capitalize(&Theme::Light), "Light");
        assert_eq!(capitalize(&Theme::Dark), "Dark");
    }

    #[test]
    fn test_theme_can_be_lowercased() {
        assert_eq!(lowercase(&Theme::Default), "default");
        assert_eq!(lowercase(&Theme::Light), "light");
        assert_eq!(lowercase(&Theme::Dark), "dark");
    }

    #[test]
    fn test_can_get_list_of_valid_themes() {
        let themes = vec![
            Theme::Default,
            Theme::Light,
            Theme::Dark,
            Theme::Cupcake,
            Theme::Bumblebee,
            Theme::Emerald,
            Theme::Corporate,
            Theme::Synthwave,
            Theme::Retro,
            Theme::Cyberpunk,
            Theme::Valentine,
            Theme::Halloween,
            Theme::Garden,
            Theme::Forest,
            Theme::Aqua,
            Theme::Lofi,
            Theme::Pastel,
            Theme::Fantasy,
            Theme::Wireframe,
            Theme::Black,
            Theme::Luxury,
            Theme::Dracula,
            Theme::Cmyk,
            Theme::Autumn,
            Theme::Business,
            Theme::Acid,
            Theme::Lemonade,
            Theme::Night,
            Theme::Coffee,
            Theme::Winter,
            Theme::Dim,
            Theme::Nord,
            Theme::Sunset,
            Theme::Caramellatte,
            Theme::Abyss,
            Theme::Silk,
        ];

        assert_eq!(Theme::iter().collect::<Vec<Theme>>(), themes);
    }
}
