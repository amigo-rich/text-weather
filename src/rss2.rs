use std::fmt;

use chrono::{DateTime, FixedOffset};
use url::Url;

pub enum ElementParserError {
    BufferEmpty,
    BufferTooLarge,
    InvalidInput,
}

pub trait ElementParser {
    const MAX_LEN: usize;
    fn new(buffer: &str) -> Result<Self, ElementParserError>
    where
        Self: std::marker::Sized;
}

#[derive(Clone, Debug)]
pub struct GeoRSSPointElement {
    latitude: f32,
    longitude: f32,
}

impl GeoRSSPointElement {
    const REQUIRED_PARTS: usize = 2;
}

impl fmt::Display for GeoRSSPointElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Latitude: {}, Longitude: {}",
            self.latitude, self.longitude
        )
    }
}

impl ElementParser for GeoRSSPointElement {
    const MAX_LEN: usize = 16;
    fn new(buffer: &str) -> Result<GeoRSSPointElement, ElementParserError> {
        if buffer.is_empty() {
            return Err(ElementParserError::BufferEmpty);
        } else if buffer.len() > Self::MAX_LEN {
            return Err(ElementParserError::BufferTooLarge);
        }
        let parts: Vec<&str> = buffer.split(' ').collect();
        if parts.is_empty() || parts.len() != Self::REQUIRED_PARTS {
            return Err(ElementParserError::InvalidInput);
        }
        let latitude: f32 = match parts[0].parse() {
            Ok(latitude) => latitude,
            Err(_) => return Err(ElementParserError::InvalidInput),
        };
        let longitude: f32 = match parts[0].parse() {
            Ok(longitude) => longitude,
            Err(_) => return Err(ElementParserError::InvalidInput),
        };
        Ok(GeoRSSPointElement {
            latitude,
            longitude,
        })
    }
}

#[derive(Clone, Debug)]
pub struct UrlElement {
    url: Url,
}

impl fmt::Display for UrlElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url.as_str())
    }
}

impl ElementParser for UrlElement {
    const MAX_LEN: usize = 256;
    fn new(buffer: &str) -> Result<UrlElement, ElementParserError> {
        if buffer.is_empty() {
            return Err(ElementParserError::BufferEmpty);
        } else if buffer.len() > Self::MAX_LEN {
            return Err(ElementParserError::BufferTooLarge);
        }
        let url = match Url::parse(buffer) {
            Ok(url) => url,
            Err(_) => return Err(ElementParserError::InvalidInput),
        };
        Ok(UrlElement { url })
    }
}

#[derive(Clone, Debug)]
pub struct DateTimeElement {
    date_time: DateTime<FixedOffset>,
}

impl fmt::Display for DateTimeElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.date_time.to_rfc2822())
    }
}

impl ElementParser for DateTimeElement {
    const MAX_LEN: usize = 128;
    fn new(buffer: &str) -> Result<DateTimeElement, ElementParserError> {
        if buffer.is_empty() {
            return Err(ElementParserError::BufferEmpty);
        } else if buffer.len() > Self::MAX_LEN {
            return Err(ElementParserError::BufferTooLarge);
        }
        let date_time = match DateTime::parse_from_rfc2822(buffer) {
            Ok(date_time) => date_time,
            Err(_) => return Err(ElementParserError::InvalidInput),
        };
        Ok(DateTimeElement { date_time })
    }
}

#[derive(Clone, Debug)]
pub struct StringElement {
    string: String,
}

impl fmt::Display for StringElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl ElementParser for StringElement {
    const MAX_LEN: usize = 256;
    fn new(buffer: &str) -> Result<StringElement, ElementParserError> {
        if buffer.is_empty() {
            return Err(ElementParserError::BufferEmpty);
        } else if buffer.len() > Self::MAX_LEN {
            return Err(ElementParserError::BufferTooLarge);
        }
        Ok(StringElement {
            string: String::from(buffer),
        })
    }
}

#[derive(Debug)]
pub struct BBCWeatherThreeDayItem {
    title: StringElement,
    link: UrlElement,
    description: StringElement,
    pub_date: DateTimeElement,
    guid: UrlElement,
    geo_rss_point: GeoRSSPointElement,
}

impl BBCWeatherThreeDayItem {
    pub fn title(&self) -> &StringElement {
        &self.title
    }
    pub fn link(&self) -> &UrlElement {
        &self.link
    }
    pub fn description(&self) -> &StringElement {
        &self.description
    }
    pub fn pub_date(&self) -> &DateTimeElement {
        &self.pub_date
    }
    pub fn guid(&self) -> &UrlElement {
        &self.guid
    }
    pub fn geo_rss_point(&self) -> &GeoRSSPointElement {
        &self.geo_rss_point
    }
}

pub struct BBCWeatherThreeDay {
    title: StringElement,
    link: UrlElement,
    description: StringElement,
    language: StringElement,
    copyright: StringElement,
    pub_date: DateTimeElement,
    items: Vec<BBCWeatherThreeDayItem>,
}

impl BBCWeatherThreeDay {
    pub fn title(&self) -> &StringElement {
        &self.title
    }
    pub fn link(&self) -> &UrlElement {
        &self.link
    }
    pub fn description(&self) -> &StringElement {
        &self.description
    }
    pub fn language(&self) -> &StringElement {
        &self.language
    }
    pub fn copyright(&self) -> &StringElement {
        &self.copyright
    }
    pub fn pub_date(&self) -> &DateTimeElement {
        &self.pub_date
    }
    pub fn items(&self) -> &Vec<BBCWeatherThreeDayItem> {
        &self.items
    }
}

pub trait Builder {
    fn new() -> Self;
}

#[derive(Debug)]
pub struct ChannelBuilder {
    title: Option<StringElement>,
    link: Option<UrlElement>,
    description: Option<StringElement>,
    language: Option<StringElement>,
    copyright: Option<StringElement>,
    pub_date: Option<DateTimeElement>,
    items: Option<Vec<BBCWeatherThreeDayItem>>,
}

impl ChannelBuilder {
    pub fn set_title(&mut self, title: StringElement) {
        self.title.replace(title);
    }
    pub fn set_link(&mut self, link: UrlElement) {
        self.link.replace(link);
    }
    pub fn set_description(&mut self, description: StringElement) {
        self.description.replace(description);
    }
    pub fn set_language(&mut self, language: StringElement) {
        self.language.replace(language);
    }
    pub fn set_copyright(&mut self, copyright: StringElement) {
        self.copyright.replace(copyright);
    }
    pub fn set_pub_date(&mut self, pub_date: DateTimeElement) {
        self.pub_date.replace(pub_date);
    }
    pub fn set_item(&mut self, item: BBCWeatherThreeDayItem) {
        if self.items.is_some() {
            let mut items = self.items.take().unwrap();
            items.push(item);
            self.items.replace(items);
        } else {
            let items = vec![item];
            self.items.replace(items);
        }
    }
    pub fn get(self) -> Result<BBCWeatherThreeDay, ()> {
        if self.title.is_none()
            || self.link.is_none()
            || self.description.is_none()
            || self.language.is_none()
            || self.copyright.is_none()
            || self.pub_date.is_none()
            || self.items.is_none()
        {
            return Err(());
        }
        Ok(BBCWeatherThreeDay {
            title: self.title.unwrap(),
            link: self.link.unwrap(),
            description: self.description.unwrap(),
            language: self.language.unwrap(),
            copyright: self.copyright.unwrap(),
            pub_date: self.pub_date.unwrap(),
            items: self.items.unwrap(),
        })
    }
}

impl Builder for ChannelBuilder {
    fn new() -> ChannelBuilder {
        ChannelBuilder {
            title: None,
            link: None,
            description: None,
            language: None,
            copyright: None,
            pub_date: None,
            items: None,
        }
    }
}

pub struct ItemBuilder {
    title: Option<StringElement>,
    link: Option<UrlElement>,
    description: Option<StringElement>,
    pub_date: Option<DateTimeElement>,
    guid: Option<UrlElement>,
    geo_rss_point: Option<GeoRSSPointElement>,
}

impl ItemBuilder {
    pub fn set_title(&mut self, title: StringElement) {
        self.title.replace(title);
    }
    pub fn set_link(&mut self, link: UrlElement) {
        self.link.replace(link);
    }
    pub fn set_description(&mut self, description: StringElement) {
        self.description.replace(description);
    }
    pub fn set_pub_date(&mut self, pub_date: DateTimeElement) {
        self.pub_date.replace(pub_date);
    }
    pub fn set_guid(&mut self, guid: UrlElement) {
        self.guid.replace(guid);
    }
    pub fn set_geo_rss_point(&mut self, geo_rss_point: GeoRSSPointElement) {
        self.geo_rss_point.replace(geo_rss_point);
    }
    pub fn get(&self) -> Result<BBCWeatherThreeDayItem, ()> {
        if self.title.is_none() || self.link.is_none() || self.description.is_none() {
            return Err(());
        }
        Ok(BBCWeatherThreeDayItem {
            title: self.title.clone().unwrap(),
            link: self.link.clone().unwrap(),
            description: self.description.clone().unwrap(),
            pub_date: self.pub_date.clone().unwrap(),
            guid: self.guid.clone().unwrap(),
            geo_rss_point: self.geo_rss_point.clone().unwrap(),
        })
    }
}

impl Builder for ItemBuilder {
    fn new() -> ItemBuilder {
        ItemBuilder {
            title: None,
            link: None,
            description: None,
            pub_date: None,
            guid: None,
            geo_rss_point: None,
        }
    }
}
