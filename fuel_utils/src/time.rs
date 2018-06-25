use std::time::Duration;

pub fn duration_to_secs(dur: Duration) -> f64 {
    dur.as_secs() as f64 + f64::from(dur.subsec_nanos()) / 1_000_000_000.0
}
