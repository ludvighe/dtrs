use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};

const NAIVE_DT_FMT: &[&str] = &[
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%dT%H:%M:%S",
    "%Y-%m-%d",
    "%H:%M:%S",
    "%H:%M",
];

const AWARE_DT_FMT: &[&str] = &[
    "%Y-%m-%d %H:%M:%S %z",
    "%Y-%m-%dT%H:%M:%S%z",
    "%Y-%m-%dT%H:%M:%S%.f%:z",
];

pub fn parse_datetime(input: &str) -> Result<DateTime<Utc>, &'static str> {
    if input == "now" {
        return Ok(Utc::now());
    }

    for &fmt in NAIVE_DT_FMT {
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(input, fmt) {
            return Ok(naive_dt.and_utc());
        } else if let Ok(date) = NaiveDate::parse_from_str(input, fmt) {
            let naive_dt = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            return Ok(naive_dt.and_utc());
        } else if let Ok(time) = NaiveTime::parse_from_str(input, fmt) {
            let today = Utc::now().date_naive();
            let naive_dt = NaiveDateTime::new(today, time);
            return Ok(naive_dt.and_utc());
        }
    }

    let norm_input = input.replace(",", ".");
    for &fmt in AWARE_DT_FMT {
        if let Ok(dt) = DateTime::parse_from_str(&norm_input, fmt) {
            return Ok(dt.with_timezone(&Utc));
        }
    }

    if let Ok(dt) = DateTime::parse_from_rfc2822(input) {
        return Ok(dt.with_timezone(&Utc));
    }
    if let Ok(dt) = DateTime::parse_from_rfc3339(input) {
        return Ok(dt.with_timezone(&Utc));
    }

    Err("Invalid time format")
}

pub fn parse_duration(input: &str) -> Result<Duration, &'static str> {
    let input = input.trim();

    if input.is_empty() || input.len() < 2 {
        return Err("Invalid duration format");
    }

    let (number_str, unit_char) = input.split_at(input.len() - 1);
    let unit = unit_char.chars().next().unwrap();

    let value = match number_str.parse::<i64>() {
        Ok(res) => res,
        _ => return Err("Invalid duration number"),
    };

    match unit {
        'd' => Ok(Duration::days(value)),
        'h' => Ok(Duration::hours(value)),
        'm' => Ok(Duration::minutes(value)),
        's' => Ok(Duration::seconds(value)),
        'w' => Ok(Duration::days(7 * value)),
        _ => Err("Invalid duration character"),
    }
}
