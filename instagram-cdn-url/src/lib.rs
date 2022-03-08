use core::{char::from_digit, fmt};

use chrono::{DateTime, NaiveDateTime, Utc};
use url::{ParseError, Url};

pub struct CdnUrl {
    pub oe_datetime: DateTime<Utc>,
}

impl CdnUrl {
    pub fn parse(url: impl AsRef<str>) -> Result<Self, CdnUrlParseError> {
        let url = url.as_ref();
        let url = Url::parse(url).map_err(CdnUrlParseError::UrlParseError)?;

        let pairs = url.query_pairs();

        let (_, oe) = pairs
            .clone()
            .find(|(k, _)| k == "oe")
            .ok_or(CdnUrlParseError::BadURLTimestamp)?;

        let oe_datetime =
            oe_string_to_datetime(&oe).map_err(|_| CdnUrlParseError::BadURLTimestamp)?;

        pairs
            .clone()
            .find(|(k, _)| k == "oh")
            .ok_or(CdnUrlParseError::BadURLHash)?;

        pairs
            .clone()
            .find(|(k, _)| k == "_nc_ohc")
            .ok_or(CdnUrlParseError::URLSignatureMismatch)?;

        pairs
            .clone()
            .find(|(k, _)| k == "_nc_ht")
            .ok_or(CdnUrlParseError::URLSignatureMismatch)?;

        Ok(Self { oe_datetime })
    }

    pub fn is_url_signature_expired(&self) -> bool {
        self.oe_datetime < Utc::now()
    }
}

//
#[derive(Debug, PartialEq)]
pub enum CdnUrlParseError {
    UrlParseError(ParseError),
    BadURLTimestamp,
    BadURLHash,
    URLSignatureMismatch,
}

impl fmt::Display for CdnUrlParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CdnUrlParseError {}

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
    (match n / r {
        0 => String::new(),
        n => to_str_radix(n, r),
    }) + &String::from(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn test_oe_string_and_datetime_converter() -> Result<(), Box<dyn error::Error>> {
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
    fn test_parse() -> Result<(), Box<dyn error::Error>> {
        let cdn_url = CdnUrl::parse( "https://scontent-lax3-1.cdninstagram.com/v/t51.2885-19/s150x150/14718046_215742295528430_4651559330867314688_a.jpg?_nc_ht=scontent-lax3-1.cdninstagram.com&_nc_ohc=n76LD7OkqcEAX_rFqpg&tp=1&oh=b68eb21889f4d6406bea1db175f16b3b&oe=600DBA0C")?;

        if Utc::now()
            > DateTime::<Utc>::from_utc(
                NaiveDateTime::new(
                    NaiveDate::from_ymd(2021, 1, 24),
                    NaiveTime::from_hms(18, 18, 52),
                ),
                Utc,
            )
        {
            assert!(cdn_url.is_url_signature_expired());
        } else {
            assert!(!cdn_url.is_url_signature_expired());
        }

        Ok(())
    }

    #[test]
    fn test_parse_when_missing_oe() {
        assert_eq!(
            CdnUrl::parse( "https://scontent-lax3-1.cdninstagram.com/v/t51.2885-19/s150x150/14718046_215742295528430_4651559330867314688_a.jpg?_nc_ht=scontent-lax3-1.cdninstagram.com&_nc_ohc=n76LD7OkqcEAX_rFqpg&tp=1&oh=b68eb21889f4d6406bea1db175f16b3b&oeFOO=600DBA0C").err(),
            Some(CdnUrlParseError::BadURLTimestamp)
        );
    }

    #[test]
    fn test_parse_when_missing_oh() {
        assert_eq!(
            CdnUrl::parse( "https://scontent-lax3-1.cdninstagram.com/v/t51.2885-19/s150x150/14718046_215742295528430_4651559330867314688_a.jpg?_nc_ht=scontent-lax3-1.cdninstagram.com&_nc_ohc=n76LD7OkqcEAX_rFqpg&tp=1&ohFOO=b68eb21889f4d6406bea1db175f16b3b&oe=600DBA0C").err(),
            Some(CdnUrlParseError::BadURLHash)
        );
    }

    #[test]
    fn test_parse_when_missing_nc_ohc() {
        assert_eq!(
            CdnUrl::parse( "https://scontent-lax3-1.cdninstagram.com/v/t51.2885-19/s150x150/14718046_215742295528430_4651559330867314688_a.jpg?_nc_ht=scontent-lax3-1.cdninstagram.com&_nc_ohcFOO=n76LD7OkqcEAX_rFqpg&tp=1&oh=b68eb21889f4d6406bea1db175f16b3b&oe=600DBA0C").err(),
            Some(CdnUrlParseError::URLSignatureMismatch)
        );
    }

    #[test]
    fn test_parse_when_missing_nc_ht() {
        assert_eq!(
            CdnUrl::parse( "https://scontent-lax3-1.cdninstagram.com/v/t51.2885-19/s150x150/14718046_215742295528430_4651559330867314688_a.jpg?_nc_htFOO=scontent-lax3-1.cdninstagram.com&_nc_ohc=n76LD7OkqcEAX_rFqpg&tp=1&oh=b68eb21889f4d6406bea1db175f16b3b&oe=600DBA0C").err(),
            Some(CdnUrlParseError::URLSignatureMismatch)
        );
    }
}
