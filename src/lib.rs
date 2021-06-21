mod error;
mod model;
use model::Forecast;
mod net;
use net::reqwest_fetch_url;
mod parser;
use parser::parse_document;

use url::Url;

pub fn run(uri: &str) {
    let url = match Url::parse(uri) {
        Ok(url) => url,
        Err(_) => panic!("Invalid url: {}", uri),
    };

    let rss_body = match reqwest_fetch_url(url) {
        Ok(rss_body) => rss_body,
        Err(_) => panic!("Network error"),
    };

    let parsed = match parse_document(rss_body.as_str()) {
        Ok(forecast) => forecast,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!();
        }
    };
    for item in parsed.get_items() {
        let forecast = Forecast::parse_from_item_title_and_description(
            item.get_title(),
            item.get_description(),
        )
        .unwrap();
        println!("{}\n{}", forecast.summary(), forecast.details());
    }
}
