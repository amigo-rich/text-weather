use crate::rss2::{DateTimeElement, GeoRSSPointElement, StringElement, UrlElement};

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
    pub fn new(
        title: StringElement,
        link: UrlElement,
        description: StringElement,
        pub_date: DateTimeElement,
        guid: UrlElement,
        geo_rss_point: GeoRSSPointElement,
    ) -> BBCWeatherThreeDayItem {
        BBCWeatherThreeDayItem {
            title,
            link,
            description,
            pub_date,
            guid,
            geo_rss_point,
        }
    }
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
    pub fn new(
        title: StringElement,
        link: UrlElement,
        description: StringElement,
        language: StringElement,
        copyright: StringElement,
        pub_date: DateTimeElement,
        items: Vec<BBCWeatherThreeDayItem>,
    ) -> BBCWeatherThreeDay {
        BBCWeatherThreeDay {
            title,
            link,
            description,
            language,
            copyright,
            pub_date,
            items,
        }
    }
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
