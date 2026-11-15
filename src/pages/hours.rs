use crate::components::BackgroundBlur;
use crate::pages::compare::sorted_cities_from_query;
use crate::timezone::City;
use crate::HOURS;
use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::Tz;
use leptos::prelude::*;
use leptos_router::hooks::query_signal;
use std::str::FromStr;

#[component]
pub fn Hours(cities_query: Memo<Option<String>>) -> impl IntoView {
    let (get_cities, set_cities) = signal(Vec::new());
    let (get_rows, set_rows) = signal(Vec::new());

    let (hours_query, set_hours_query) = query_signal::<usize>(HOURS);

    // Listen for the `zone` url query to change and when it does, re-render the cities.
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = cities_query.get().unwrap_or_default();
        let cities = sorted_cities_from_query(query);

        set_rows.set(get_hours(&cities, hours_query.get()));
        set_cities.set(cities);
    });

    view! {
        <BackgroundBlur>
            <div class="flex justify-center m-4">
                <table class="table table-fixed text-center">
                    <thead>
                        <tr>
                            <For
                                each=move || get_cities.get()
                                key=|city| city.slug.clone()
                                children=move |city| {
                                    let c = city.clone();
                                    view! {
                                        <th>{c.name}</th>
                                    }
                                }
                            />
                        </tr>
                    </thead>
                    <tbody>
                        {
                            move || get_rows
                                .get()
                                .into_iter()
                                .map(|row| view! { <tr class="hover:bg-base-300">{
                                    row.into_iter()
                                        .map(|cell| view! { <td>{format!("{}:00", cell)}</td>})
                                        .collect_view()
                                    }</tr>})
                                .collect_view()
                        }
                </tbody>
              </table>
            </div>
        </BackgroundBlur>

        <div class="flex justify-center m-4">
            <BackgroundBlur>
                <button
                    type="button"
                    id="hours_table_more_button"
                    aria-label="Button to show or hide more timezone hours"
                    class="btn btn-soft"
                    on:click=move |_| {
                        let hours = toggle_hours(hours_query.get());
                        set_hours_query.set(Some(hours))
                    }
                >
                    {move || if hours_query.get() == None || hours_query.get() == Some(5) {
                        "Show More"
                    } else {
                        "Show Less"
                    }}

                </button>
            </BackgroundBlur>
        </div>
    }
}

fn get_hours(cities: &Vec<City>, rows_count_query: Option<usize>) -> Vec<Vec<u32>> {
    let mut table_rows = new_table_rows(rows_count_query);

    for city in cities {
        let tz = Tz::from_str(&city.timezone).unwrap_or(Tz::UTC);
        add_hours(Utc::now().with_timezone(&tz), &mut table_rows);
    }

    table_rows
}

fn new_table_rows(rows_count_query: Option<usize>) -> Vec<Vec<u32>> {
    let mut table_rows = Vec::new();

    let rows_count = match rows_count_query {
        Some(rows_count) => rows_count,
        None => 5,
    };

    for _ in 0..rows_count {
        table_rows.push(Vec::new());
    }

    table_rows
}

fn add_hours(mut local_time: DateTime<Tz>, table_rows: &mut Vec<Vec<u32>>) {
    // so the loop shows the current hour at the top of the table
    local_time = local_time - Duration::hours(1);

    for row in table_rows {
        local_time = local_time + Duration::hours(1);
        row.push(local_time.hour());
    }
}

fn toggle_hours(current_hours: Option<usize>) -> usize {
    match current_hours {
        Some(hours) => {
            if hours == 24 {
                5
            } else {
                24
            }
        }
        None => 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_hours_to_5_with_no_input() {
        assert_eq!(toggle_hours(None), 5);
    }

    #[test]
    fn test_toggle_hours_to_5_with_24_hour_input() {
        assert_eq!(toggle_hours(Some(24)), 5);
    }

    #[test]
    fn test_toggle_hours_to_24_with_5_hour_input() {
        assert_eq!(toggle_hours(Some(5)), 24);
    }

    #[test]
    fn test_initialise_table_of_5_rows() {
        let answer: Vec<Vec<u32>> = vec![vec![], vec![], vec![], vec![], vec![]];
        assert_eq!(new_table_rows(Some(5)), answer);
    }

    #[test]
    fn test_initialise_table_of_24_rows() {
        let answer: Vec<Vec<u32>> = vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        assert_eq!(new_table_rows(Some(24)), answer);
    }

    #[test]
    fn test_can_generate_table_content_for_london_with_5_rows() {
        let local_time = Utc
            .with_ymd_and_hms(2026, 11, 20, 19, 30, 10)
            .unwrap()
            .with_timezone(&Tz::Europe__London);
        let mut table_rows = vec![vec![], vec![], vec![], vec![], vec![]];

        add_hours(local_time, &mut table_rows);

        assert_eq!(
            table_rows,
            vec![vec![19], vec![20], vec![21], vec![22], vec![23]]
        );
    }

    #[test]
    fn test_can_generate_table_content_for_paris_with_5_rows() {
        let local_time = Utc
            .with_ymd_and_hms(2026, 11, 20, 19, 30, 10)
            .unwrap()
            .with_timezone(&Tz::Europe__Paris);
        let mut table_rows = vec![vec![], vec![], vec![], vec![], vec![]];

        add_hours(local_time, &mut table_rows);

        assert_eq!(
            table_rows,
            vec![vec![20], vec![21], vec![22], vec![23], vec![0]]
        );
    }

    #[test]
    fn test_can_generate_table_content_for_london_with_24_rows() {
        let local_time = Utc
            .with_ymd_and_hms(2026, 11, 20, 19, 30, 10)
            .unwrap()
            .with_timezone(&Tz::Europe__London);
        let mut table_rows = vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        add_hours(local_time, &mut table_rows);

        assert_eq!(
            table_rows,
            vec![
                vec![19],
                vec![20],
                vec![21],
                vec![22],
                vec![23],
                vec![0],
                vec![1],
                vec![2],
                vec![3],
                vec![4],
                vec![5],
                vec![6],
                vec![7],
                vec![8],
                vec![9],
                vec![10],
                vec![11],
                vec![12],
                vec![13],
                vec![14],
                vec![15],
                vec![16],
                vec![17],
                vec![18]
            ]
        );
    }

    #[test]
    fn test_can_add_paris_column_to_hours_table() {
        let local_time = Utc
            .with_ymd_and_hms(2026, 11, 20, 19, 30, 10)
            .unwrap()
            .with_timezone(&Tz::Europe__Paris);
        let mut table_rows = vec![vec![19], vec![20], vec![21], vec![22], vec![23]];

        add_hours(local_time, &mut table_rows);

        assert_eq!(
            table_rows,
            vec![
                vec![19, 20],
                vec![20, 21],
                vec![21, 22],
                vec![22, 23],
                vec![23, 0]
            ]
        );
    }
}
