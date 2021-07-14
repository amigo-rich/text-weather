pub mod dayoption;
use dayoption::Day;
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

pub fn run(uri: &str, day_option: Option<Day>) -> Result<(), Error> {
    let url = build_url(uri)?;

    let rss_body = reqwest_fetch_url(url)?;

    let parsed = parse_document(rss_body.as_str())?;

    let forecast = Forecast::parse_from_items(parsed.get_items())?;
    if day_option.is_some() {
        let day_option = day_option.unwrap();
        let forecast = match day_option {
            Day::Today => forecast.one()?,
            Day::Tomorrow => forecast.two()?,
            Day::Overmorrow => forecast.three()?,
        };
        println!("{}", day_option);
        println!("{}", forecast.summary());
        println!("{}", forecast.details());
    } else {
        for (day, name) in forecast
            .into_iter()
            .zip(["Today", "Tomorrow", "Overmorrow"].iter())
        {
            println!("{}", name);
            println!("{}", day.summary());
            println!("{}", day.details());
        }
    }
    Ok(())
}
