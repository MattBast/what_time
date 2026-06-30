use crate::timezone::{to_slug, City, CITIES};
use chrono_tz::Tz;

/// Parse a list of timezones into a `current_time` url query.
pub fn tz_vec_to_url_query(timezones: Vec<Tz>) -> String {
    timezones
        .iter()
        .map(|tz| tz.to_string().replace("/", "__"))
        .collect::<Vec<String>>()
        .join(",")
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
    use super::*;

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
