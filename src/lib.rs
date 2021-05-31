mod net;
mod rss2;
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

    let forecast = match parse_document(rss_body.as_str()) {
        Ok(forecast) => forecast,
        Err(e) => panic!("Something went wrong in parsing...{:?}", e),
    };

    println!("{} {}", forecast.title(), forecast.description());
    println!("{}", forecast.link());
    println!("{}", forecast.language());
    println!("{}", forecast.copyright());
    println!("{}", forecast.pub_date());

    for item in forecast.items() {
        println!("{} {}", item.title(), item.description());
        println!("{}", item.pub_date());
        println!("{}", item.link());
        println!("{}", item.guid());
        println!("{}", item.geo_rss_point());
    }
}
