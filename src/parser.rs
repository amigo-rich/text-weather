use crate::convert::{to_date_time, to_geo_rss_point, to_url, valid_str};
use crate::rss::{BBCWeatherLatest, ItemBuilder, RssBuilder};
use quick_xml::events::Event;
use quick_xml::Reader;

const RSS_EL: &[u8] = b"rss";
const TITLE_EL: &[u8] = b"title";
const LINK_EL: &[u8] = b"link";
const DESCRIPTION_EL: &[u8] = b"description";
const LANGUAGE_EL: &[u8] = b"language";
const COPYRIGHT_EL: &[u8] = b"copyright";
const PUBDATE_EL: &[u8] = b"pubDate";
const ITEM_EL: &[u8] = b"item";
const GUID_EL: &[u8] = b"guid";
const GEORSS_POINT_EL: &[u8] = b"georss:point";

#[derive(PartialEq)]
enum ParseState {
    Initial,
    Rss,
    Title,
    Link,
    Description,
    Language,
    Copyright,
    PubDate,
    Item,
    Complete,
}

enum ItemParseState {
    Initial,
    Title,
    Link,
    Description,
    PubDate,
    Guid,
    GeoRssPoint,
}

pub enum Error {
    Parse,
}

pub fn parse_document(body: &str) -> Result<BBCWeatherLatest, Error> {
    let mut reader = Reader::from_str(body);
    let mut buffer = Vec::new();
    let mut text_buffer = String::new();
    let mut parse_state = ParseState::Initial;
    let mut item_parse_state = ItemParseState::Initial;
    let mut builder = RssBuilder::new();
    let mut item_builder = ItemBuilder::new();

    while parse_state != ParseState::Complete {
        match reader.read_event(&mut buffer) {
            Ok(Event::Start(ref e)) => match e.name() {
                RSS_EL => parse_state = ParseState::Rss,
                TITLE_EL => match parse_state {
                    ParseState::Item => item_parse_state = ItemParseState::Title,
                    _ => parse_state = ParseState::Title,
                },
                LINK_EL => match parse_state {
                    ParseState::Item => item_parse_state = ItemParseState::Link,
                    _ => parse_state = ParseState::Link,
                },
                DESCRIPTION_EL => match parse_state {
                    ParseState::Item => item_parse_state = ItemParseState::Description,
                    _ => parse_state = ParseState::Description,
                },
                LANGUAGE_EL => parse_state = ParseState::Language,
                COPYRIGHT_EL => parse_state = ParseState::Copyright,
                PUBDATE_EL => match parse_state {
                    ParseState::Item => item_parse_state = ItemParseState::PubDate,
                    _ => parse_state = ParseState::PubDate,
                },
                ITEM_EL => {
                    parse_state = ParseState::Item;
                    item_parse_state = ItemParseState::Initial;
                }
                GUID_EL => item_parse_state = ItemParseState::Guid,
                GEORSS_POINT_EL => item_parse_state = ItemParseState::GeoRssPoint,
                _ => (),
            },
            Ok(Event::Text(e)) => {
                text_buffer = match e.unescape_and_decode(&reader) {
                    Ok(text_buffer) => text_buffer,
                    Err(_) => return Err(Error::Parse),
                };
            }
            Ok(Event::End(ref e)) => match e.name() {
                RSS_EL => parse_state = ParseState::Complete,
                TITLE_EL => {
                    if !valid_str(&text_buffer) {
                        return Err(Error::Parse);
                    }
                    match parse_state {
                        ParseState::Item => item_builder.set_title(&text_buffer),
                        _ => builder.set_title(&text_buffer),
                    }
                }
                LINK_EL => {
                    let link = match to_url(&text_buffer) {
                        Ok(link) => link,
                        Err(_) => return Err(Error::Parse),
                    };
                    match parse_state {
                        ParseState::Item => item_builder.set_link(link),
                        _ => builder.set_link(link),
                    }
                }
                DESCRIPTION_EL => {
                    if !valid_str(&text_buffer) {
                        return Err(Error::Parse);
                    }
                    match parse_state {
                        ParseState::Item => item_builder.set_description(&text_buffer),
                        _ => builder.set_description(&text_buffer),
                    }
                }
                LANGUAGE_EL => {
                    if !valid_str(&text_buffer) {
                        return Err(Error::Parse);
                    }
                    match parse_state {
                        ParseState::Language => builder.set_language(&text_buffer),
                        _ => return Err(Error::Parse),
                    }
                }
                COPYRIGHT_EL => {
                    if !valid_str(&text_buffer) {
                        return Err(Error::Parse);
                    }
                    match parse_state {
                        ParseState::Copyright => builder.set_copyright(&text_buffer),
                        _ => return Err(Error::Parse),
                    }
                }
                PUBDATE_EL => {
                    let pub_date = match to_date_time(&text_buffer) {
                        Ok(pub_date) => pub_date,
                        Err(_) => return Err(Error::Parse),
                    };
                    match parse_state {
                        ParseState::Item => item_builder.set_pub_date(pub_date),
                        _ => builder.set_pub_date(pub_date),
                    }
                }
                ITEM_EL => match parse_state {
                    ParseState::Item => {
                        let item = match item_builder.get() {
                            Ok(item) => item,
                            Err(_) => return Err(Error::Parse),
                        };
                        builder.set_item(item);
                    }
                    _ => return Err(Error::Parse),
                },
                GUID_EL => match parse_state {
                    ParseState::Item => {
                        let guid = match to_url(&text_buffer) {
                            Ok(guid) => guid,
                            Err(_) => return Err(Error::Parse),
                        };
                        match item_parse_state {
                            ItemParseState::Guid => item_builder.set_guid(guid),
                            _ => return Err(Error::Parse),
                        }
                    }
                    _ => {
                        return Err(Error::Parse);
                    }
                },
                GEORSS_POINT_EL => match parse_state {
                    ParseState::Item => {
                        let geo_rss = match to_geo_rss_point(&text_buffer) {
                            Ok(geo_rss) => geo_rss,
                            Err(_) => return Err(Error::Parse),
                        };
                        match item_parse_state {
                            ItemParseState::GeoRssPoint => {
                                item_builder.set_geo_rss_point(geo_rss);
                            }
                            _ => return Err(Error::Parse),
                        }
                    }
                    _ => return Err(Error::Parse),
                },
                _ => (),
            },
            Ok(Event::Eof) => (),
            Err(e) => {
                eprintln!("Error at position {}: {:?}", reader.buffer_position(), e);
                return Err(Error::Parse);
            }
            _ => (),
        }
    }
    match builder.get() {
        Ok(forecast) => Ok(forecast),
        Err(_) => Err(Error::Parse),
    }
}
