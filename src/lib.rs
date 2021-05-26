use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug)]
enum RssParseState {
    Initial,
    Rss,
    Channel,
    Title,
    Link,
    Description,
    Language,
    Copyright,
    PubDate,
    DcDate,
    DcLanguage,
    DcRights,
    Item,
    Complete,
}

#[derive(Debug)]
enum RssItemParseState {
    Initial,
    Title,
    Link,
    Description,
    PubDate,
    Guid,
    DcDate,
    GeorssPoint,
}

#[derive(Debug)]
pub struct Item {
    title: String,
    link: String,
    description: String,
    //pub_date: String,
    //guid: String,
    //dc_date: String,
    //georss: String,
}

pub struct ItemBuilder {
    title: String,
    link: String,
    description: String,
}

#[derive(Debug)]
enum ErrorType {
    ItemBuilderError,
}

impl ItemBuilder {
    pub fn set_title(&mut self, title: &str) {
        self.title.push_str(title);
    }
    pub fn set_link(&mut self, link: &str) {
        self.link.push_str(link);
    }
    pub fn set_description(&mut self, description: &str) {
        self.description.push_str(description);
    }
    fn get(&self) -> Result<Item, ErrorType> {
        if self.title.is_empty() || self.link.is_empty() || self.description.is_empty() {
            return Err(ErrorType::ItemBuilderError);
        }
        Ok(Item {
            title: self.title.clone(),
            link: self.link.clone(),
            description: self.description.clone(),
        })
    }
    pub fn new() -> ItemBuilder {
        ItemBuilder {
            title: String::new(),
            link: String::new(),
            description: String::new(),
        }
    }
}

pub fn run(uri: &str) {
    let rss_body = reqwest::blocking::get(uri).unwrap().text().unwrap();

    println!("{:?}", rss_body);
    println!("Going in for a parse job!");

    let mut reader = Reader::from_str(rss_body.as_str());

    let mut buf = Vec::new();
    let mut text_buf = String::new();
    let mut rss_parse_state = RssParseState::Initial;
    let mut rss_item_parse_state = RssItemParseState::Initial;
    let mut builder = ItemBuilder::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"rss" => rss_parse_state = RssParseState::Rss,
                b"channel" => rss_parse_state = RssParseState::Channel,
                b"title" => match rss_parse_state {
                    RssParseState::Item => {
                        rss_item_parse_state = RssItemParseState::Title;
                    }
                    _ => {
                        rss_parse_state = RssParseState::Title;
                    }
                },
                b"link" => match rss_parse_state {
                    RssParseState::Item => {
                        rss_item_parse_state = RssItemParseState::Link;
                    }
                    _ => {
                        rss_parse_state = RssParseState::Link;
                    }
                },
                b"description" => match rss_parse_state {
                    RssParseState::Item => {
                        rss_item_parse_state = RssItemParseState::Description;
                    }
                    _ => {
                        rss_parse_state = RssParseState::Description;
                    }
                },
                b"language" => rss_parse_state = RssParseState::Language,
                b"copyright" => rss_parse_state = RssParseState::Copyright,
                b"pubDate" => match rss_parse_state {
                    RssParseState::Item => {
                        rss_item_parse_state = RssItemParseState::PubDate;
                    }
                    _ => {
                        rss_parse_state = RssParseState::PubDate;
                    }
                },
                b"dc:date" => match rss_parse_state {
                    RssParseState::Item => {
                        rss_item_parse_state = RssItemParseState::DcDate;
                    }
                    _ => {
                        rss_parse_state = RssParseState::DcDate;
                    }
                },
                b"dc:language" => rss_parse_state = RssParseState::DcLanguage,
                b"dc:rights" => rss_parse_state = RssParseState::DcRights,
                b"item" => rss_parse_state = RssParseState::Item,
                b"guid" => match rss_parse_state {
                    RssParseState::Item => {
                        rss_item_parse_state = RssItemParseState::Guid;
                    }
                    _ => {
                        eprintln!(
                            "Warning: {} outside of item",
                            str::from_utf8(e.name()).unwrap()
                        );
                    }
                },
                b"georss:point" => match rss_parse_state {
                    RssParseState::Item => {
                        rss_item_parse_state = RssItemParseState::GeorssPoint;
                    }
                    _ => {
                        eprintln!(
                            "Warning: {} outside of item",
                            str::from_utf8(e.name()).unwrap()
                        );
                    }
                },
                _ => {
                    println!("Unhandled tag: {}", str::from_utf8(e.name()).unwrap());
                }
            },
            Ok(Event::Text(e)) => {
                text_buf = e.unescape_and_decode(&reader).unwrap();
            }
            Ok(Event::End(ref e)) => match e.name() {
                b"rss" => rss_parse_state = RssParseState::Complete,
                b"channel" => (),
                b"title" => match rss_parse_state {
                    RssParseState::Item => (),
                    _ => builder.set_title(text_buf.as_str()),
                },
                b"link" => match rss_parse_state {
                    RssParseState::Item => (),
                    _ => builder.set_link(text_buf.as_str()),
                },
                b"description" => match rss_parse_state {
                    RssParseState::Item => (),
                    _ => builder.set_description(text_buf.as_str()),
                },
                b"language" => (),
                b"copyright" => (),
                b"pubDate" => match rss_parse_state {
                    RssParseState::Item => (),
                    _ => (),
                },
                b"dc:date" => match rss_parse_state {
                    RssParseState::Item => (),
                    _ => (),
                },
                b"dc:language" => (),
                b"dc:rights" => (),
                b"item" => (),
                b"guid" => match rss_parse_state {
                    RssParseState::Item => (),
                    _ => {
                        eprintln!(
                            "Warning: {} outside of item",
                            str::from_utf8(e.name()).unwrap()
                        );
                    }
                },
                b"georss:point" => match rss_parse_state {
                    RssParseState::Item => (),
                    _ => {
                        eprintln!(
                            "Warning: {} outside of item",
                            str::from_utf8(e.name()).unwrap()
                        );
                    }
                },
                _ => {
                    println!("Unhandled tag: {}", str::from_utf8(e.name()).unwrap());
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        println!("Rss Parser is: {:?}", rss_parse_state);
        println!("Rss ITem Parser is: {:?}", rss_item_parse_state);
        buf.clear();
    }
    let outcome = builder.get().unwrap();
    println!("Item: {:?}", outcome);
}
