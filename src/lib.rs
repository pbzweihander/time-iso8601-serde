#![no_std]

extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::ToString;

use serde::de::{Deserialize, Deserializer, Error as DeError};
use serde::ser::Serializer;

pub const fn date_to_iso8601(date: time::Date) -> iso8601::Date {
    let (year, month, day) = date.as_ymd();
    iso8601::Date::YMD {
        year,
        month: month as u32,
        day: day as u32,
    }
}

const WEEKDAYS: [time::Weekday; 7] = [
    time::Weekday::Monday,
    time::Weekday::Tuesday,
    time::Weekday::Wednesday,
    time::Weekday::Thursday,
    time::Weekday::Friday,
    time::Weekday::Saturday,
    time::Weekday::Sunday,
];

pub const fn date_from_iso8601(
    date: iso8601::Date,
) -> Result<time::Date, time::error::ComponentRange> {
    match date {
        iso8601::Date::YMD { year, month, day } => {
            time::Date::try_from_ymd(year, month as u8, day as u8)
        }
        iso8601::Date::Week { year, ww, d } => {
            time::Date::try_from_iso_ywd(year, ww as u8, WEEKDAYS[d as usize - 1])
        }
        iso8601::Date::Ordinal { year, ddd } => time::Date::try_from_yo(year, ddd as u16),
    }
}

pub const fn time_to_iso8601(time: time::Time, offset: time::UtcOffset) -> iso8601::Time {
    let offset_minutes = offset.as_minutes();
    iso8601::Time {
        hour: time.hour() as u32,
        minute: time.minute() as u32,
        second: time.second() as u32,
        millisecond: time.millisecond() as u32,
        tz_offset_hours: offset_minutes as i32 / 60,
        tz_offset_minutes: offset_minutes as i32 % 60,
    }
}

pub const fn time_from_iso8601(
    iso8601::Time {
        hour,
        minute,
        second,
        millisecond,
        tz_offset_hours,
        tz_offset_minutes,
    }: iso8601::Time,
) -> (
    Result<time::Time, time::error::ComponentRange>,
    time::UtcOffset,
) {
    (
        time::Time::try_from_hms_milli(hour as u8, minute as u8, second as u8, millisecond as u16),
        time::UtcOffset::minutes(tz_offset_hours as i16 * 60 + tz_offset_minutes as i16),
    )
}

pub fn datetime_to_iso8601(datetime: time::OffsetDateTime) -> iso8601::DateTime {
    let date = datetime.date();
    let time = datetime.time();
    let offset = datetime.offset();
    iso8601::DateTime {
        date: date_to_iso8601(date),
        time: time_to_iso8601(time, offset),
    }
}

pub fn datetime_from_iso8601(
    iso8601::DateTime { date, time }: iso8601::DateTime,
) -> Result<time::OffsetDateTime, time::error::ComponentRange> {
    let (time, offset) = time_from_iso8601(time);
    Ok(date_from_iso8601(date)?
        .with_time(time?)
        .assume_offset(offset))
}

pub mod datetime {
    use super::*;

    pub fn serialize<S>(time: &time::OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&datetime_to_iso8601(*time).to_string())
    }

    pub fn deserialize<'de, D>(d: D) -> Result<time::OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        iso8601::datetime(Cow::<'_, str>::deserialize(d)?.as_ref())
            .map_err(DeError::custom)
            .and_then(|time| datetime_from_iso8601(time).map_err(DeError::custom))
    }

    pub mod optional {
        use super::*;

        pub fn serialize<S>(
            time: &Option<time::OffsetDateTime>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match time {
                Some(time) => serializer.serialize_str(&datetime_to_iso8601(*time).to_string()),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(d: D) -> Result<Option<time::OffsetDateTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let optional = Option::<Cow<'_, str>>::deserialize(d)?;
            match optional {
                Some(s) => iso8601::datetime(s.as_ref())
                    .map_err(DeError::custom)
                    .and_then(|time| datetime_from_iso8601(time).map_err(DeError::custom))
                    .map(Some),
                None => Ok(None),
            }
        }
    }
}

pub mod date {
    use super::*;

    pub fn serialize<S>(time: &time::Date, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date_to_iso8601(*time).to_string())
    }

    pub fn deserialize<'de, D>(d: D) -> Result<time::Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        iso8601::date(Cow::<'_, str>::deserialize(d)?.as_ref())
            .map_err(DeError::custom)
            .and_then(|time| date_from_iso8601(time).map_err(DeError::custom))
    }

    pub mod optional {
        use super::*;

        pub fn serialize<S>(time: &Option<time::Date>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match time {
                Some(time) => serializer.serialize_str(&date_to_iso8601(*time).to_string()),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(d: D) -> Result<Option<time::Date>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let optional = Option::<Cow<'_, str>>::deserialize(d)?;
            match optional {
                Some(s) => iso8601::date(s.as_ref())
                    .map_err(DeError::custom)
                    .and_then(|time| date_from_iso8601(time).map_err(DeError::custom))
                    .map(Some),
                None => Ok(None),
            }
        }
    }
}

pub mod time_offset {
    use super::*;

    pub fn serialize<S>(
        (time, offset): &(time::Time, time::UtcOffset),
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&time_to_iso8601(*time, *offset).to_string())
    }

    pub fn deserialize<'de, D>(d: D) -> Result<(time::Time, time::UtcOffset), D::Error>
    where
        D: Deserializer<'de>,
    {
        iso8601::time(Cow::<'_, str>::deserialize(d)?.as_ref())
            .map_err(DeError::custom)
            .and_then(|time| {
                let (time, offset) = time_from_iso8601(time);
                time.map_err(DeError::custom).map(|t| (t, offset))
            })
    }

    pub mod optional {
        use super::*;

        pub fn serialize<S>(
            time: &Option<(time::Time, time::UtcOffset)>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match time {
                Some((time, offset)) => {
                    serializer.serialize_str(&time_to_iso8601(*time, *offset).to_string())
                }
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(d: D) -> Result<Option<(time::Time, time::UtcOffset)>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let optional = Option::<Cow<'_, str>>::deserialize(d)?;
            match optional {
                Some(s) => iso8601::time(s.as_ref())
                    .map_err(DeError::custom)
                    .and_then(|time| {
                        let (time, offset) = time_from_iso8601(time);
                        time.map_err(DeError::custom).map(|t| Some((t, offset)))
                    }),
                None => Ok(None),
            }
        }
    }
}
