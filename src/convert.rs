use std::str;

use crate::rss::GeoRssPoint;
use chrono::{DateTime, FixedOffset};
use url::Url;

const FIELD_MAX_LEN: usize = 256;

#[derive(Debug, PartialEq)]
pub enum Error {
    Conversion,
}

pub fn valid_str(s: &str) -> bool {
    if s.is_empty() || s.len() > FIELD_MAX_LEN {
        return false;
    }
    true
}

pub fn to_date_time(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    if !valid_str(s) {
        return Err(Error::Conversion);
    }
    let result = match DateTime::parse_from_rfc2822(s) {
        Ok(parsed) => parsed,
        Err(_) => return Err(Error::Conversion),
    };

    Ok(result)
}

pub fn to_url(s: &str) -> Result<Url, Error> {
    if !valid_str(s) {
        return Err(Error::Conversion);
    }
    let url = match Url::parse(s) {
        Ok(parsed) => parsed,
        Err(_) => return Err(Error::Conversion),
    };

    Ok(url)
}

pub fn to_geo_rss_point(s: &str) -> Result<GeoRssPoint, Error> {
    if !valid_str(s) {
        return Err(Error::Conversion);
    }
    let collected: Vec<&str> = s.split(' ').collect();
    if collected.is_empty() || collected.len() != 2 {
        return Err(Error::Conversion);
    }
    match GeoRssPoint::new(collected[0], collected[1]) {
        Ok(point) => Ok(point),
        Err(_) => Err(Error::Conversion),
    }
}
