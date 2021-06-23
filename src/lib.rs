mod error;
mod model;
use model::Forecast;
mod net;
use net::reqwest_fetch_url;
mod parser;
use parser::parse_document;

use std::io::{stdin, stdout};
use std::thread;
use std::time::Duration;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

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
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!();
        }
    };
    let forecast = Forecast::parse_from_items(parsed.get_items()).unwrap();
    gui(&forecast);
}

pub fn gui(_f: &Forecast) {
    let _ = stdout().into_raw_mode().unwrap();
    let mut index = 0;
    loop {
        let stdin = stdin();
        /* key input */
        let ev = stdin.events().next().unwrap();
        match ev.unwrap() {
            Event::Key(k) => match k {
                Key::Char('q') => return,
                Key::Left => {
                    if index == 0 {
                        index = 2;
                    } else {
                        index = index - 1;
                    }
                }
                Key::Right => {
                    if index == 2 {
                        index = 0;
                    } else {
                        index = index + 1;
                    }
                }
                _ => (),
            },
            _ => break,
        }
        thread::sleep(Duration::from_millis(50));
    }
}
