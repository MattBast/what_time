use crate::timezone::{to_slug, City, CITIES};
use chrono_tz::Tz;
use std::str::FromStr;

pub fn url_query_to_timezones(query: String) -> Vec<Tz> {
    let mut zones = Vec::new();

    for zone_str in query.split(",") {
        match Tz::from_str(&zone_str.replace("__", "/")) {
            Ok(zone) => zones.push(zone),
            Err(_) => continue,
        }
    }

    zones
}

pub fn city_slug_to_timezone(slug: &str) -> Option<Tz> {
    match normalize_city_slug(slug).as_str() {
        "london" => Some(Tz::Europe__London),
        "paris" => Some(Tz::Europe__Paris),
        "new-york" => Some(Tz::America__New_York),
        "tokyo" => Some(Tz::Asia__Tokyo),
        "sydney" => Some(Tz::Australia__Sydney),
        "san-francisco" => Some(Tz::America__Los_Angeles), // <-- Needs update later to display San Francisco on the card title
        "los-angeles" => Some(Tz::America__Los_Angeles),
        "chicago" => Some(Tz::America__Chicago),
        "toronto" => Some(Tz::America__Toronto),
        "vancouver" => Some(Tz::America__Vancouver),
        "mexico-city" => Some(Tz::America__Mexico_City),
        "sao-paulo" => Some(Tz::America__Sao_Paulo),
        "utc" => Some(Tz::UTC),
        "dublin" => Some(Tz::Europe__Dublin),
        "berlin" => Some(Tz::Europe__Berlin),
        "amsterdam" => Some(Tz::Europe__Amsterdam),
        "madrid" => Some(Tz::Europe__Madrid),
        "rome" => Some(Tz::Europe__Rome),
        "zurich" => Some(Tz::Europe__Zurich),
        "dubai" => Some(Tz::Asia__Dubai),
        "singapore" => Some(Tz::Asia__Singapore),
        "hong-kong" => Some(Tz::Asia__Hong_Kong),
        "shanghai" => Some(Tz::Asia__Shanghai),
        "seoul" => Some(Tz::Asia__Seoul),
        "mumbai" => Some(Tz::Asia__Kolkata),
        "delhi" => Some(Tz::Asia__Kolkata),
        "kolkata" => Some(Tz::Asia__Kolkata),
        "bangkok" => Some(Tz::Asia__Bangkok),
        "jakarta" => Some(Tz::Asia__Jakarta),
        "auckland" => Some(Tz::Pacific__Auckland),
        "melbourne" => Some(Tz::Australia__Melbourne),
        "brisbane" => Some(Tz::Australia__Brisbane),
        "perth" => Some(Tz::Australia__Perth),
        "cairo" => Some(Tz::Africa__Cairo),
        "johannesburg" => Some(Tz::Africa__Johannesburg),
        "lagos" => Some(Tz::Africa__Lagos),
        _ => None,
    }
}

pub fn city_slugs_to_url_query(slugs: &[String]) -> String {
    let mut timezones = Vec::new();

    for slug in slugs {
        if let Some(timezone) = city_slug_to_timezone(slug) {
            if !timezones.contains(&timezone) {
                timezones.push(timezone);
            }
        }
    }

    tz_vec_to_url_query(timezones)
}

pub fn route_cities_url_query(cities_param: Option<String>, extra: Option<String>) -> String {
    let slugs_str = route_cities_slugs(cities_param, extra);
    let slugs: Vec<String> = slugs_str
        .split(',')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    city_slugs_to_url_query(&slugs)
}

/// Get all the timezones from the "zone" query of the url and remove them from
/// the list of timezones that fill the timezone select element.
pub fn remove_timezone(query: String, variants: &mut Vec<Tz>) {
    for zone_str in query.split(",") {
        match Tz::from_str(&zone_str.replace("__", "/")) {
            Ok(zone) => {
                if let Some(index) = variants.iter().position(|tz| *tz == zone) {
                    variants.remove(index);
                }
            }
            Err(_) => continue,
        }
    }
}

/// Parse a list of timezones into a `current_time` url query.
pub fn tz_vec_to_url_query(timezones: Vec<Tz>) -> String {
    timezones
        .iter()
        .map(|tz| tz.to_string().replace("/", "__"))
        .collect::<Vec<String>>()
        .join(",")
}

fn normalize_city_slug(slug: &str) -> String {
    slug.trim()
        .to_lowercase()
        .replace('_', "-")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

/// Take the `current_time` query and add a new timezone to it.
pub fn add_timezone_to_url_query(url_query: Option<String>, tz: Tz) -> String {
    let mut timezones = url_query_to_timezones(url_query.unwrap_or_default());
    timezones.push(tz);

    tz_vec_to_url_query(timezones)
}

/// Take the `current_time` query and remove the specified timezone from it.
pub fn remove_timezone_from_url_query(url_query: Option<String>, tz: Tz) -> String {
    let mut timezones = url_query_to_timezones(url_query.unwrap_or_default());
    timezones.retain(|value| *value != tz);

    tz_vec_to_url_query(timezones)
}

pub fn find_city_by_slug(slug: &str) -> Option<&City> {
    let slug = slug.to_lowercase();
    let overridden_slug = match slug.as_str() {
        "new-york" => "new-york-city",
        "sao-paulo" => "são-paulo",
        "bogota" => "bogotá",
        "quezon" => "quezon-city",
        _ => &slug,
    };

    if let Some(city) = CITIES.iter().find(|city| city.slug == *overridden_slug) {
        return Some(city);
    }

    // Check by name directly
    if let Some(city) = CITIES
        .iter()
        .find(|city| to_slug(&city.name) == *overridden_slug)
    {
        return Some(city);
    }

    // Fallback: check if slug matches a timezone name (e.g. Europe__London)
    let tz_name = overridden_slug.replace("__", "/");
    if let Some(city) = CITIES
        .iter()
        .find(|city| city.timezone.to_lowercase() == tz_name.to_lowercase())
    {
        return Some(city);
    }

    None
}

pub fn url_query_to_cities(query: String) -> Vec<City> {
    let mut cities = Vec::new();
    for slug in query.split(",") {
        let slug = slug.trim();
        if !slug.is_empty() {
            if let Some(city) = find_city_by_slug(slug) {
                if !cities.contains(city) {
                    cities.push(city.clone());
                }
            }
        }
    }
    cities
}

pub fn add_city_to_url_query(url_query: Option<String>, city_slug: &str) -> String {
    let mut slugs: Vec<String> = url_query
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if !slugs.contains(&city_slug.to_string()) {
        slugs.push(city_slug.to_string());
    }
    slugs.join(",")
}

pub fn remove_city_from_url_query(url_query: Option<String>, city_slug: &str) -> String {
    let mut slugs: Vec<String> = url_query
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    slugs.retain(|s| s != city_slug);
    slugs.join(",")
}

pub fn route_cities_slugs(cities_param: Option<String>, extra: Option<String>) -> String {
    let mut slugs = Vec::new();
    if let Some(param) = cities_param {
        for segment in param.split('/') {
            let segment = segment.trim();
            if !segment.is_empty() {
                slugs.push(segment.to_string());
            }
        }
    }
    if let Some(ext) = extra {
        for slug in ext.split(',') {
            let slug = slug.trim();
            if !slug.is_empty() {
                slugs.push(slug.to_string());
            }
        }
    }
    slugs.join(",")
}

#[cfg(test)]
mod tests {
    use chrono_tz::TZ_VARIANTS;

    use super::*;

    #[test]
    fn test_url_query_to_timezones() {
        let timezones =
            url_query_to_timezones("Europe__London,Europe__Paris,America__Atikokan".to_string());

        assert_eq!(
            timezones,
            vec![Tz::Europe__London, Tz::Europe__Paris, Tz::America__Atikokan]
        );
    }

    #[test]
    fn test_url_query_to_timezones_with_extra_commas() {
        let timezones = url_query_to_timezones(
            ",,,,Europe__London,Europe__Paris,America__Atikokan".to_string(),
        );

        assert_eq!(
            timezones,
            vec![Tz::Europe__London, Tz::Europe__Paris, Tz::America__Atikokan]
        );
    }

    #[test]
    fn test_no_timezones_parsed_into_url_query() {
        let new_url_query = tz_vec_to_url_query(vec![]);
        assert_eq!(new_url_query, "".to_string());
    }

    #[test]
    fn test_one_timezone_parsed_into_url_query() {
        let new_url_query = tz_vec_to_url_query(vec![Tz::Europe__Amsterdam]);
        assert_eq!(new_url_query, "Europe__Amsterdam".to_string());
    }

    #[test]
    fn test_two_timezones_parsed_into_url_query() {
        let new_url_query = tz_vec_to_url_query(vec![Tz::Europe__Amsterdam, Tz::Europe__Andorra]);
        assert_eq!(
            new_url_query,
            "Europe__Amsterdam,Europe__Andorra".to_string()
        );
    }

    #[test]
    fn test_add_timezone_to_url_query() {
        let new_url_query = add_timezone_to_url_query(
            Some("Europe__London,Europe__Paris".into()),
            Tz::Europe__Dublin,
        );

        assert_eq!(
            new_url_query,
            "Europe__London,Europe__Paris,Europe__Dublin".to_string()
        );
    }

    #[test]
    fn test_add_timezone_to_empty_url_query() {
        let new_url_query = add_timezone_to_url_query(None, Tz::Europe__Dublin);

        assert_eq!(new_url_query, "Europe__Dublin".to_string());
    }

    #[test]
    fn test_add_timezone_to_url_query_with_extra_commas() {
        let new_url_query =
            add_timezone_to_url_query(Some("Europe__London,,,".into()), Tz::Europe__Dublin);

        assert_eq!(new_url_query, "Europe__London,Europe__Dublin".to_string());
    }

    #[test]
    fn test_remove_timezone_from_url_query() {
        let new_url_query = remove_timezone_from_url_query(
            Some("Europe__London,Europe__Paris,Europe__Dublin".into()),
            Tz::Europe__Dublin,
        );

        assert_eq!(new_url_query, "Europe__London,Europe__Paris".to_string());
    }

    #[test]
    fn test_add_second_timezone_builds_comma_separated_zone_query() {
        let zone = add_timezone_to_url_query(Some("Africa__Abidjan".into()), Tz::Africa__Accra);
        assert_eq!(zone, "Africa__Abidjan,Africa__Accra");
    }

    #[test]
    fn test_variants_filtered_by_url_query() {
        let mut all_timezones = TZ_VARIANTS.clone().to_vec();

        remove_timezone(
            "Europe__London,Europe__Paris,Europe__Dublin".into(),
            &mut all_timezones,
        );

        assert_eq!(all_timezones.len(), TZ_VARIANTS.len() - 3);
        assert!(!all_timezones.contains(&Tz::Europe__London));
        assert!(!all_timezones.contains(&Tz::Europe__Paris));
        assert!(!all_timezones.contains(&Tz::Europe__Dublin));
    }

    #[test]
    fn test_city_slug_to_timezone_supports_core_seo_pairs() {
        assert_eq!(city_slug_to_timezone("london"), Some(Tz::Europe__London));
        assert_eq!(city_slug_to_timezone("paris"), Some(Tz::Europe__Paris));
        assert_eq!(
            city_slug_to_timezone("new-york"),
            Some(Tz::America__New_York)
        );
        assert_eq!(city_slug_to_timezone("tokyo"), Some(Tz::Asia__Tokyo));
        assert_eq!(city_slug_to_timezone("sydney"), Some(Tz::Australia__Sydney));
        assert_eq!(
            city_slug_to_timezone("san-francisco"),
            Some(Tz::America__Los_Angeles)
        );
    }

    #[test]
    fn test_route_cities_url_query_builds_zone_query_from_slugs() {
        let query = route_cities_url_query(Some("london/paris".into()), None);
        assert_eq!(query, "Europe__London,Europe__Paris");
    }

    #[test]
    fn test_route_cities_url_query_includes_extra_city_slugs() {
        let query = route_cities_url_query(
            Some("london/new-york".into()),
            Some("paris,tokyo,sydney".into()),
        );

        assert_eq!(
            query,
            "Europe__London,America__New_York,Europe__Paris,Asia__Tokyo,Australia__Sydney"
        );
    }

    #[test]
    fn test_route_cities_url_query_ignores_unknown_slugs_and_duplicates() {
        let query = route_cities_url_query(
            Some("london/not-a-city".into()),
            Some("london,paris".into()),
        );

        assert_eq!(query, "Europe__London,Europe__Paris");
    }

    #[test]
    fn test_find_city_by_slug() {
        let london = find_city_by_slug("london").unwrap();
        assert_eq!(london.name, "London");
        assert_eq!(london.timezone, "Europe/London");

        // check slug override
        let nyc = find_city_by_slug("new-york").unwrap();
        assert_eq!(nyc.name, "New York City");

        // check legacy timezone fallback
        let london_legacy = find_city_by_slug("Europe__London").unwrap();
        assert_eq!(london_legacy.name, "London");
    }

    #[test]
    fn test_url_query_to_cities() {
        let cities = url_query_to_cities("london,new-york,Europe__Paris".to_string());
        assert_eq!(cities.len(), 3);
        assert_eq!(cities[0].name, "London");
        assert_eq!(cities[1].name, "New York City");
        assert_eq!(cities[2].name, "Paris");
    }

    #[test]
    fn test_add_remove_city_from_url_query() {
        let q = add_city_to_url_query(None, "london");
        assert_eq!(q, "london");

        let q2 = add_city_to_url_query(Some("london".to_string()), "paris");
        assert_eq!(q2, "london,paris");

        let q3 = remove_city_from_url_query(Some("london,paris".to_string()), "london");
        assert_eq!(q3, "paris");
    }

    #[test]
    fn test_route_cities_slugs() {
        let q = route_cities_slugs(
            Some("london/paris".to_string()),
            Some("new-york,tokyo".to_string()),
        );
        assert_eq!(q, "london,paris,new-york,tokyo");
    }
}
