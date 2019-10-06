use chrono::{DateTime, NaiveDateTime, offset::Utc};

macro_rules! try_opt {
    ($e:expr) => ( match $e {
        Some(e) => e,
        None => {
            return None;
        },
    });
}

pub fn epoch_to_iso(epoch: i32) -> String {
    // Chrono is a little silly and can't easily convert from epoch to utc timezone
    let d: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(epoch as i64, 0), Utc);
    d.to_rfc3339()
}

pub fn iso_to_epoch(iso: &str) -> u32 {
    DateTime::parse_from_rfc3339(iso).map(|x| x.timestamp() as u32).unwrap_or(0)
}
