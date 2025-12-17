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
}
