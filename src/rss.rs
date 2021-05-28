use chrono::{DateTime, FixedOffset};
use url::Url;

pub enum Error {
    Converstion,
    GeoRssPoint,
}

#[derive(Debug, Clone)]
pub struct GeoRssPoint {
    latitude: f32,
    longitude: f32,
}

impl GeoRssPoint {
    pub fn new(latitude: &str, longitude: &str) -> Result<GeoRssPoint, Error> {
        let latitude: f32 = match latitude.parse() {
            Ok(latitude) => latitude,
            Err(_) => return Err(Error::GeoRssPoint),
        };
        let longitude: f32 = match longitude.parse() {
            Ok(longitude) => longitude,
            Err(_) => return Err(Error::GeoRssPoint),
        };
        Ok(GeoRssPoint {
            latitude,
            longitude,
        })
    }
}

#[derive(Debug)]
pub struct BBCWeatherLatestItem {
    title: String,
    link: Url,
    description: String,
    pub_date: DateTime<FixedOffset>,
    guid: Url,
    geo_rss_point: GeoRssPoint,
}

impl BBCWeatherLatestItem {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn pub_date(&self) -> String {
        self.pub_date.to_rfc2822()
    }
}

#[derive(Debug)]
pub struct ItemBuilder {
    title: Option<String>,
    link: Option<Url>,
    description: Option<String>,
    pub_date: Option<DateTime<FixedOffset>>,
    guid: Option<Url>,
    geo_rss_point: Option<GeoRssPoint>,
}

impl ItemBuilder {
    pub fn new() -> ItemBuilder {
        ItemBuilder {
            title: None,
            link: None,
            description: None,
            pub_date: None,
            guid: None,
            geo_rss_point: None,
        }
    }
    pub fn set_title(&mut self, title: &str) {
        self.title = Some(String::from(title));
    }
    pub fn set_link(&mut self, link: Url) {
        self.link.replace(link);
    }
    pub fn set_description(&mut self, description: &str) {
        self.description.replace(String::from(description));
    }
    pub fn set_pub_date(&mut self, pub_date: DateTime<FixedOffset>) {
        self.pub_date.replace(pub_date);
    }
    pub fn set_guid(&mut self, guid: Url) {
        self.guid.replace(guid);
    }
    pub fn set_geo_rss_point(&mut self, geo_rss_point: GeoRssPoint) {
        self.geo_rss_point.replace(geo_rss_point);
    }
    pub fn get(&self) -> Result<BBCWeatherLatestItem, Error> {
        if self.title.is_none()
            || self.link.is_none()
            || self.description.is_none()
            || self.pub_date.is_none()
            || self.guid.is_none()
            || self.geo_rss_point.is_none()
        {
            return Err(Error::Converstion);
        }
        Ok(BBCWeatherLatestItem {
            title: self.title.clone().unwrap(),
            link: self.link.clone().unwrap(),
            description: self.description.clone().unwrap(),
            pub_date: self.pub_date.clone().unwrap(),
            guid: self.guid.clone().unwrap(),
            geo_rss_point: self.geo_rss_point.clone().unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct BBCWeatherLatest {
    title: String,
    link: Url,
    description: String,
    language: String,
    copyright: String,
    pub_date: DateTime<FixedOffset>,
    items: Vec<BBCWeatherLatestItem>,
}

impl BBCWeatherLatest {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn items(&self) -> &Vec<BBCWeatherLatestItem> {
        &self.items
    }
}

pub struct RssBuilder {
    title: Option<String>,
    link: Option<Url>,
    description: Option<String>,
    language: Option<String>,
    copyright: Option<String>,
    pub_date: Option<DateTime<FixedOffset>>,
    items: Option<Vec<BBCWeatherLatestItem>>,
}

impl RssBuilder {
    pub fn new() -> RssBuilder {
        RssBuilder {
            title: None,
            link: None,
            description: None,
            language: None,
            copyright: None,
            pub_date: None,
            items: None,
        }
    }
    pub fn set_title(&mut self, title: &str) {
        self.title.replace(String::from(title));
    }
    pub fn set_link(&mut self, link: Url) {
        self.link.replace(link);
    }
    pub fn set_description(&mut self, description: &str) {
        self.description.replace(String::from(description));
    }
    pub fn set_language(&mut self, language: &str) {
        self.language.replace(String::from(language));
    }
    pub fn set_copyright(&mut self, copyright: &str) {
        self.copyright.replace(String::from(copyright));
    }
    pub fn set_pub_date(&mut self, pub_date: DateTime<FixedOffset>) {
        self.pub_date.replace(pub_date);
    }
    pub fn set_item(&mut self, item: BBCWeatherLatestItem) {
        if self.items.is_some() {
            let mut items = self.items.take().unwrap();
            items.push(item);
            self.items.replace(items);
        } else {
            let items = vec![item];
            self.items.replace(items);
        }
    }
    pub fn get(self) -> Result<BBCWeatherLatest, Error> {
        if self.title.is_none()
            || self.link.is_none()
            || self.description.is_none()
            || self.language.is_none()
            || self.copyright.is_none()
            || self.pub_date.is_none()
            || self.items.is_none()
        {
            return Err(Error::Converstion);
        }
        Ok(BBCWeatherLatest {
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
