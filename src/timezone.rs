use chrono::prelude::*;
use chrono::DateTime;
use chrono_tz::Tz;

/// Get a timestamp with the UTC timezone. Convert it to the specified timezone.
/// If `current_time` is None or not a valid Unix timestamp, default the time to now.
pub fn utc_to_local_timezone(current_time: Option<i64>, tz: Tz) -> DateTime<Tz> {
    match current_time {
        Some(timestamp) => DateTime::from_timestamp(timestamp, 0)
            .map(|dt| dt.with_timezone(&tz))
            .unwrap_or_else(|| Utc::now().with_timezone(&tz)),
        None => Utc::now().with_timezone(&tz),
    }
}

/// Sort a list of timezones with the westernmost timezones at the start of the list
/// and the eastern most at the end.
pub fn sort_timezones(timezones: &mut [Tz]) {
    let naive_date_time = Utc::now().naive_utc();

    timezones.sort_by_key(|tz| {
        tz.offset_from_utc_datetime(&naive_date_time)
            .fix()
            .local_minus_utc()
    });
}

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct City {
    pub name: String,
    pub country: String,
    pub timezone: String,
    pub emoji: String,
    pub slug: String,
}

impl<'de> serde::Deserialize<'de> for City {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (name, country, timezone, emoji) =
            <(String, String, String, String)>::deserialize(deserializer)?;
        let slug = to_slug(&name);
        Ok(City {
            name,
            country,
            timezone,
            emoji,
            slug,
        })
    }
}

pub fn to_slug(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace(['_', '/', ' '], "-")
        .split('-')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub static CITIES: LazyLock<Vec<City>> = LazyLock::new(|| {
    let json_str = include_str!("data/cities.json");
    let mut raw_cities: Vec<City> =
        serde_json::from_str(json_str).expect("Failed to parse cities.json");

    // We want to generate unique slugs.
    // Count occurrences of base slugs to see which ones need country suffix.
    let mut base_slug_counts = HashMap::new();
    for city in &raw_cities {
        *base_slug_counts.entry(city.slug.clone()).or_insert(0) += 1;
    }

    // Resolve collisions
    let mut slug_to_index = HashMap::new();
    for city in &mut raw_cities {
        let mut base_slug = city.slug.clone();
        if base_slug_counts[&base_slug] > 1 {
            base_slug = format!("{}-{}", base_slug, to_slug(&city.country));
        }

        let mut final_slug = base_slug.clone();
        let mut counter = 1;
        while slug_to_index.contains_key(&final_slug) {
            counter += 1;
            final_slug = format!("{base_slug}-{counter}");
        }

        slug_to_index.insert(final_slug.clone(), ());
        city.slug = final_slug;
    }

    raw_cities
});

pub fn sort_cities(cities: &mut [City]) {
    let naive_date_time = Utc::now().naive_utc();
    cities.sort_by_key(|city| {
        let tz = Tz::from_str(&city.timezone).unwrap_or(Tz::UTC);
        tz.offset_from_utc_datetime(&naive_date_time)
            .fix()
            .local_minus_utc()
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_gmt() {
        let gmt_time = utc_to_local_timezone(Some(1765920530), Tz::Europe__London);

        assert_eq!(
            gmt_time,
            DateTime::parse_from_str("2025-12-16 21:28:50 +0000", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .with_timezone(&Tz::Europe__London)
        );
    }

    #[test]
    fn test_parse_to_cet() {
        let gmt_time = utc_to_local_timezone(Some(1765920530), Tz::Europe__Paris);

        assert_eq!(
            gmt_time,
            DateTime::parse_from_str("2025-12-16 22:28:50 +0100", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .with_timezone(&Tz::Europe__Paris)
        );
    }

    #[test]
    fn test_parse_to_ist() {
        let gmt_time = utc_to_local_timezone(Some(1765920530), Tz::Asia__Calcutta);

        assert_eq!(
            gmt_time,
            DateTime::parse_from_str("2025-12-17 02:58:50 +0530", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .with_timezone(&Tz::Asia__Calcutta)
        );
    }

    #[test]
    fn test_invalid_timestamp_defaults_to_now() {
        let from_invalid = utc_to_local_timezone(Some(i64::MAX), Tz::UTC);
        let from_none = utc_to_local_timezone(None, Tz::UTC);

        let epoch = DateTime::from_timestamp(0, 0).expect("epoch should be valid");
        assert!(from_invalid > epoch);
        assert!(from_none > epoch);
        assert!((from_invalid - from_none).num_seconds().abs() < 2);
    }

    #[test]
    fn test_sorting_timezones() {
        let mut timezones = vec![Tz::Europe__Paris, Tz::Europe__London, Tz::America__Atikokan];
        sort_timezones(&mut timezones);

        assert_eq!(
            timezones,
            vec![Tz::America__Atikokan, Tz::Europe__London, Tz::Europe__Paris]
        );
    }

    #[test]
    fn test_cities_loading_and_slugs() {
        assert!(CITIES.len() > 1000);
        let london = CITIES
            .iter()
            .find(|c| c.slug == "london-united-kingdom" || c.slug == "london");
        assert!(london.is_some());
        let new_york = CITIES
            .iter()
            .find(|c| c.slug == "new-york-city" || c.slug == "new-york");
        assert!(new_york.is_some());
    }

    #[test]
    fn test_sort_cities() {
        let mut sample = vec![
            CITIES.iter().find(|c| c.name == "Paris").unwrap().clone(),
            CITIES
                .iter()
                .find(|c| c.name == "New York City")
                .unwrap()
                .clone(),
            CITIES.iter().find(|c| c.name == "Tokyo").unwrap().clone(),
        ];
        sort_cities(&mut sample);
        assert_eq!(sample[0].name, "New York City");
        assert_eq!(sample[1].name, "Paris");
        assert_eq!(sample[2].name, "Tokyo");
    }
}
