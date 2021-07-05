mod error;
use error::Error;
mod model;
use model::Forecast;
mod net;
use net::reqwest_fetch_url;
mod parser;
use parser::parse_document;

const URL_PART: &str = "https://weather-broker-cdn.api.bbci.co.uk/en/forecast/rss/3day";

fn build_url(segment: &str) -> Result<url::Url, Error> {
    if segment.is_empty() || segment.len() > 128 {
        return Err(Error::InvalidSegment);
    }
    Ok(url::Url::parse(&format!("{}/{}", URL_PART, segment))?)
}

pub fn run(uri: &str) -> Result<(), Error> {
    let url = build_url(uri)?;

    let rss_body = reqwest_fetch_url(url)?;

    let parsed = parse_document(rss_body.as_str())?;

    let forecast = Forecast::parse_from_items(parsed.get_items())?;
    for day in forecast {
        println!("{}", day.summary());
        println!("{}", day.details());
    }

    Ok(())
}
