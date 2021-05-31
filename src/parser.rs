use crate::rss2::{
    BBCWeatherThreeDay, Builder, ChannelBuilder, DateTimeElement, ElementParser,
    GeoRSSPointElement, ItemBuilder, StringElement, UrlElement,
};
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

#[derive(Debug)]
pub enum Error {
    TextBuffer,
    ParseTitle,
    ParseLink,
    ParseDescription,
    ParseLanguage,
    ParseCopyright,
    ParsePubDate,
    ParseItem,
    ParseGuid,
    ParseGeoRssPoint,
    LibraryError,
    BuilderError,
}

pub fn parse_document(body: &str) -> Result<BBCWeatherThreeDay, Error> {
    let mut reader = Reader::from_str(body);
    let mut buffer = Vec::new();
    let mut text_buffer = String::new();
    let mut parse_state = ParseState::Initial;
    let mut item_parse_state = ItemParseState::Initial;
    let mut builder = ChannelBuilder::new();
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
                    Err(_) => return Err(Error::TextBuffer),
                };
            }
            Ok(Event::End(ref e)) => match e.name() {
                RSS_EL => parse_state = ParseState::Complete,
                TITLE_EL => {
                    let title_el = match StringElement::new(&text_buffer) {
                        Ok(title_el) => title_el,
                        Err(_) => return Err(Error::ParseTitle),
                    };
                    match parse_state {
                        ParseState::Item => item_builder.set_title(title_el),
                        _ => builder.set_title(title_el),
                    }
                }
                LINK_EL => {
                    let link_el = match UrlElement::new(&text_buffer) {
                        Ok(link_el) => link_el,
                        Err(_) => return Err(Error::ParseLink),
                    };
                    match parse_state {
                        ParseState::Item => item_builder.set_link(link_el),
                        _ => builder.set_link(link_el),
                    }
                }
                DESCRIPTION_EL => {
                    let de = match StringElement::new(&text_buffer) {
                        Ok(de) => de,
                        Err(_) => return Err(Error::ParseDescription),
                    };
                    match parse_state {
                        ParseState::Item => item_builder.set_description(de),
                        _ => builder.set_description(de),
                    }
                }
                LANGUAGE_EL => {
                    let le = match StringElement::new(&text_buffer) {
                        Ok(le) => le,
                        Err(_) => return Err(Error::ParseLanguage),
                    };
                    match parse_state {
                        ParseState::Language => builder.set_language(le),
                        _ => return Err(Error::ParseLanguage),
                    }
                }
                COPYRIGHT_EL => {
                    let ce = match StringElement::new(&text_buffer) {
                        Ok(ce) => ce,
                        Err(_) => return Err(Error::ParseCopyright),
                    };
                    match parse_state {
                        ParseState::Copyright => builder.set_copyright(ce),
                        _ => return Err(Error::ParseCopyright),
                    }
                }
                PUBDATE_EL => {
                    let pe = match DateTimeElement::new(&text_buffer) {
                        Ok(pe) => pe,
                        Err(_) => return Err(Error::ParsePubDate),
                    };
                    match parse_state {
                        ParseState::Item => item_builder.set_pub_date(pe),
                        _ => builder.set_pub_date(pe),
                    }
                }
                ITEM_EL => match parse_state {
                    ParseState::Item => {
                        let item = match item_builder.get() {
                            Ok(item) => item,
                            Err(_) => return Err(Error::ParseItem),
                        };
                        builder.set_item(item);
                    }
                    _ => return Err(Error::ParseItem),
                },
                GUID_EL => match parse_state {
                    ParseState::Item => {
                        let ge = match UrlElement::new(&text_buffer) {
                            Ok(ge) => ge,
                            Err(_) => return Err(Error::ParseGuid),
                        };
                        match item_parse_state {
                            ItemParseState::Guid => item_builder.set_guid(ge),
                            _ => return Err(Error::ParseGuid),
                        }
                    }
                    _ => {
                        return Err(Error::ParseGuid);
                    }
                },
                GEORSS_POINT_EL => match parse_state {
                    ParseState::Item => {
                        let ge = match GeoRSSPointElement::new(&text_buffer) {
                            Ok(ge) => ge,
                            Err(_) => return Err(Error::ParseGeoRssPoint),
                        };
                        match item_parse_state {
                            ItemParseState::GeoRssPoint => {
                                item_builder.set_geo_rss_point(ge);
                            }
                            _ => return Err(Error::ParseGeoRssPoint),
                        }
                    }
                    _ => return Err(Error::ParseGeoRssPoint),
                },
                _ => (),
            },
            Ok(Event::Eof) => (),
            Err(e) => {
                eprintln!("Error at position {}: {:?}", reader.buffer_position(), e);
                return Err(Error::LibraryError);
            }
            _ => (),
        }
    }
    match builder.get() {
        Ok(forecast) => Ok(forecast),
        Err(_) => Err(Error::BuilderError),
    }
}
