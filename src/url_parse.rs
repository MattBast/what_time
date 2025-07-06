use chrono_tz::Tz;
use std::str::FromStr;

/// timezones are found in the url queries written like the name of the [Tz]
/// enum. So the timezone "Europe/London" would be found as "zone=Europe__London".
/// As the query can only be one string, they are stored as a comma seperated
/// value so that multiple zones can exist in the url. If London and Paris exist
/// in the query for example, it would look like this "zone=Europe__London,Europe__Paris".
pub fn url_query_to_time_increments(query: String) -> Vec<Tz> {
    let mut zones = Vec::new();

    for zone_str in query.split(",") {
        match Tz::from_str(&zone_str.replace("__", "/")) {
            Ok(zone) => zones.push(zone),
            Err(_) => continue,
        }
    }

    zones
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
