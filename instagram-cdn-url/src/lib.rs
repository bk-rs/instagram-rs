use std::char::from_digit;
use std::fmt;

use chrono::{DateTime, NaiveDateTime, Utc};
use url::{ParseError, Url};

pub struct CdnUrl {
    pub oe_datetime: DateTime<Utc>,
}

pub enum CdnUrlError {
    UrlParseError(ParseError),
    BadURLTimestamp,
    BadURLHash,
    URLSignatureMismatch,
}

impl fmt::Display for CdnUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UrlParseError(err) => write!(f, "UrlParseError {}", err),
            Self::BadURLTimestamp => write!(f, "BadURLTimestamp"),
            Self::BadURLHash => write!(f, "BadURLHash"),
            Self::URLSignatureMismatch => write!(f, "URLSignatureMismatch"),
        }
    }
}

impl CdnUrl {
    pub fn new(url: impl AsRef<str>) -> Result<Self, CdnUrlError> {
        let url = url.as_ref();
        let url = Url::parse(url).map_err(CdnUrlError::UrlParseError)?;

        let pairs = &url.query_pairs();

        let (_, oe) = pairs
            .filter(|(k, _)| k == "oe")
            .next()
            .ok_or(CdnUrlError::BadURLTimestamp)?;

        let oe_datetime = oe_string_to_datetime(&oe).map_err(|_| CdnUrlError::BadURLTimestamp)?;

        pairs
            .filter(|(k, _)| k == "oh")
            .next()
            .ok_or(CdnUrlError::BadURLHash)?;

        pairs
            .filter(|(k, _)| k == "_nc_ohc")
            .next()
            .ok_or(CdnUrlError::URLSignatureMismatch)?;

        pairs
            .filter(|(k, _)| k == "_nc_ht")
            .next()
            .ok_or(CdnUrlError::URLSignatureMismatch)?;

        Ok(Self { oe_datetime })
    }

    pub fn is_url_signature_expired(&self) -> bool {
        self.oe_datetime < Utc::now()
    }
}

/// Ref https://steveridout.github.io/mongo-object-time/
pub fn oe_string_to_datetime(oe: impl AsRef<str>) -> Result<DateTime<Utc>, String> {
    let oe_timestamp = u32::from_str_radix(oe.as_ref(), 16).map_err(|_| "invalid".to_owned())?;

    let oe_datetime =
        DateTime::from_utc(NaiveDateTime::from_timestamp(oe_timestamp as i64, 0), Utc);

    Ok(oe_datetime)
}

pub fn oe_datetime_to_string(oe_datetime: DateTime<Utc>) -> String {
    let oe_timestamp = oe_datetime.timestamp() as u32;

    to_str_radix(oe_timestamp, 16).to_uppercase()
}

/// https://stackoverflow.com/questions/50277050/is-there-a-built-in-function-that-converts-a-number-to-a-string-in-any-base
fn to_str_radix(n: u32, r: u32) -> String {
    let c = from_digit(n % r, r).unwrap_or('!');
    return match n / r {
        0 => String::new(),
        n => to_str_radix(n, r),
    } + &String::from(c);
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn test_oe_string_and_datetime_converter() -> Result<(), String> {
        assert_eq!(
            oe_string_to_datetime("600DBA0C")?,
            DateTime::<Utc>::from_utc(
                NaiveDateTime::new(
                    NaiveDate::from_ymd(2021, 1, 24),
                    NaiveTime::from_hms(18, 18, 52)
                ),
                Utc
            )
        );

        assert_eq!(
            oe_datetime_to_string(DateTime::<Utc>::from_utc(
                NaiveDateTime::new(
                    NaiveDate::from_ymd(2021, 1, 24),
                    NaiveTime::from_hms(18, 18, 52)
                ),
                Utc
            )),
            "600DBA0C"
        );

        Ok(())
    }

    #[test]
    fn test_cdn_url() -> Result<(), String> {
        let cdn_url = CdnUrl::new( "https://scontent-lax3-1.cdninstagram.com/v/t51.2885-19/s150x150/14718046_215742295528430_4651559330867314688_a.jpg?_nc_ht=scontent-lax3-1.cdninstagram.com&_nc_ohc=n76LD7OkqcEAX_rFqpg&tp=1&oh=b68eb21889f4d6406bea1db175f16b3b&oe=600DBA0C").map_err(|err| err.to_string())?;

        if Utc::now()
            > DateTime::<Utc>::from_utc(
                NaiveDateTime::new(
                    NaiveDate::from_ymd(2021, 1, 24),
                    NaiveTime::from_hms(18, 18, 52),
                ),
                Utc,
            )
        {
            assert_eq!(cdn_url.is_url_signature_expired(), true);
        } else {
            assert_eq!(cdn_url.is_url_signature_expired(), false);
        }

        Ok(())
    }
}
