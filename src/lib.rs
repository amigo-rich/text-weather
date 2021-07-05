mod error;
mod model;
use model::Forecast;
mod net;
use net::reqwest_fetch_url;
mod parser;
use parser::parse_document;

pub fn run(uri: &str) {
    let url = match url::Url::parse(uri) {
        Ok(url) => url,
        Err(_) => panic!("Invalid url: {}", uri),
    };

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
