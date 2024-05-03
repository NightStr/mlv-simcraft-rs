use chrono::Duration;
use rust_decimal::prelude::ToPrimitive;

pub mod thieving;
pub mod fighting;


fn format_duration_as_hms(input_seconds: f64) -> String {
    let seconds = match input_seconds.to_i64() {
        Some(value) => value,
        None => panic!("Failed to convert Decimal to u64"),
    };

    let duration = Duration::seconds(seconds);
    format!(
        "{:02}:{:02}:{:02}",
        duration.num_hours(),
        duration.num_minutes() % 60,
        duration.num_seconds() % 60
    )
}