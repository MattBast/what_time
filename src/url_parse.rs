use chrono_tz::Tz;
use std::str::FromStr;

use crate::timezone::TimeIncrement;

/// timezones are found in the url queries written like the name of the [Tz]
/// enum. So the timezone "Europe/London" would be found as "zone=Europe__London".
/// As the query can only be one string, they are stored as a comma seperated
/// value so that multiple zones can exist in the url. If London and Paris exist
/// in the query for example, it would look like this "zone=Europe__London,Europe__Paris".
pub fn url_query_to_time_increments(
    query: String,
    current_time: Option<i64>,
) -> Vec<TimeIncrement> {
    let zones = url_query_to_timezones(query);

    zones
        .iter()
        .map(|timezone| match current_time {
            Some(timestamp) => TimeIncrement::from_timestamp(timestamp, *timezone),
            None => TimeIncrement::now(*timezone),
        })
        .collect()
}

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

#[cfg(test)]
mod tests {
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
    fn test_url_query_to_time_increments() {
        let increments = url_query_to_time_increments(
            "Europe__London,Europe__Paris,America__Atikokan".to_string(),
            None,
        );

        assert_eq!(
            increments,
            vec![
                TimeIncrement::now(Tz::Europe__London),
                TimeIncrement::now(Tz::Europe__Paris),
                TimeIncrement::now(Tz::America__Atikokan)
            ]
        );
    }
}
