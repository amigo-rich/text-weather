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

pub fn run(uri: &str) {
    let url = build_url(uri).unwrap();

    let rss_body = match reqwest_fetch_url(url) {
        Ok(rss_body) => rss_body,
        Err(_) => panic!("Network error"),
    };

    let parsed = match parse_document(rss_body.as_str()) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!();
        }
    };
    let forecast = Forecast::parse_from_items(parsed.get_items()).unwrap();
    for day in forecast {
        println!("{}", day.summary());
        println!("{}", day.details());
    }
}
