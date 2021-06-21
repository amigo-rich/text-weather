use crate::error::Error;

use chrono::{DateTime, FixedOffset};
use quick_xml::events::Event;
use quick_xml::Reader;
use url::Url;

const RSS_EL: &[u8] = b"rss";
const TITLE_EL: &[u8] = b"title";
const LINK_EL: &[u8] = b"link";
const DESCRIPTION_EL: &[u8] = b"description";
const LANGUAGE_EL: &[u8] = b"language";
const COPYRIGHT_EL: &[u8] = b"copyright";
const PUBDATE_EL: &[u8] = b"pubDate";
const ITEM_EL: &[u8] = b"item";
const IMAGE_EL: &[u8] = b"image";
const GUID_EL: &[u8] = b"guid";
const URL_EL: &[u8] = b"url";

#[derive(Clone, Debug)]
pub struct Channel {
    title: String,
    link: Url,
    description: String,
    language: String,
    copyright: String,
    pub_date: DateTime<FixedOffset>,
    image: Image,
    items: Vec<Item>,
}

impl Channel {
    pub fn get_items(&self) -> &Vec<Item> {
        &self.items
    }
}

#[derive(Clone, Debug)]
pub struct Image {
    title: String,
    url: Url,
    link: Url,
}

#[derive(Clone, Debug)]
pub struct Item {
    title: String,
    link: Url,
    description: String,
    pub_date: DateTime<FixedOffset>,
    guid: Url,
}

impl Item {
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
}

pub struct ChannelBuilder {
    title: Option<String>,
    link: Option<Url>,
    description: Option<String>,
    language: Option<String>,
    copyright: Option<String>,
    pub_date: Option<DateTime<FixedOffset>>,
    image: Option<Image>,
    items: Option<Vec<Item>>,
}

impl ChannelBuilder {
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
    pub fn set_image(&mut self, image: Image) {
        self.image.replace(image);
    }
    pub fn set_items(&mut self, item: Item) {
        let mut items = match self.items.is_some() {
            true => self.items.take().unwrap(),
            false => Vec::new(),
        };
        items.push(item);
        self.items = Some(items);
    }
    pub fn build(&mut self) -> Channel {
        Channel {
            title: self.title.take().unwrap(),
            link: self.link.take().unwrap(),
            description: self.description.take().unwrap(),
            language: self.language.take().unwrap(),
            copyright: self.copyright.take().unwrap(),
            pub_date: self.pub_date.take().unwrap(),
            image: self.image.take().unwrap(),
            items: self.items.take().unwrap(),
        }
    }
}

impl Default for ChannelBuilder {
    fn default() -> Self {
        ChannelBuilder {
            title: None,
            link: None,
            description: None,
            language: None,
            copyright: None,
            pub_date: None,
            image: None,
            items: None,
        }
    }
}

pub struct ImageBuilder {
    title: Option<String>,
    url: Option<Url>,
    link: Option<Url>,
}

impl ImageBuilder {
    pub fn set_title(&mut self, title: &str) {
        self.title.replace(String::from(title));
    }
    pub fn set_url(&mut self, url: Url) {
        self.url.replace(url);
    }
    pub fn set_link(&mut self, link: Url) {
        self.link.replace(link);
    }
    pub fn build(&mut self) -> Image {
        Image {
            title: self.title.take().unwrap(),
            url: self.url.take().unwrap(),
            link: self.link.take().unwrap(),
        }
    }
}

impl Default for ImageBuilder {
    fn default() -> Self {
        ImageBuilder {
            title: None,
            url: None,
            link: None,
        }
    }
}

pub struct ItemBuilder {
    title: Option<String>,
    link: Option<Url>,
    description: Option<String>,
    pub_date: Option<DateTime<FixedOffset>>,
    guid: Option<Url>,
}

impl ItemBuilder {
    pub fn set_title(&mut self, title: &str) {
        self.title.replace(String::from(title));
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
    pub fn build(&mut self) -> Item {
        Item {
            title: self.title.take().unwrap(),
            link: self.link.take().unwrap(),
            description: self.description.take().unwrap(),
            pub_date: self.pub_date.take().unwrap(),
            guid: self.guid.take().unwrap(),
        }
    }
}

impl Default for ItemBuilder {
    fn default() -> Self {
        ItemBuilder {
            title: None,
            link: None,
            description: None,
            pub_date: None,
            guid: None,
        }
    }
}

pub enum Destination {
    Channel,
    Image,
    Item,
}

pub struct Proxy {
    channel_builder: ChannelBuilder,
    image_builder: ImageBuilder,
    item_builder: ItemBuilder,
}

impl Proxy {
    pub fn new() -> Proxy {
        Proxy {
            channel_builder: ChannelBuilder::default(),
            image_builder: ImageBuilder::default(),
            item_builder: ItemBuilder::default(),
        }
    }
    pub fn set_title(&mut self, destination: &Destination, title: &str) {
        match destination {
            Destination::Channel => self.channel_builder.set_title(title),
            Destination::Image => self.image_builder.set_title(title),
            Destination::Item => self.item_builder.set_title(title),
        }
    }
    pub fn set_link(&mut self, destination: &Destination, link: &str) -> Result<(), Error> {
        let link = TryFrom::<Url>::get(&link)?;
        match destination {
            Destination::Channel => self.channel_builder.set_link(link),
            Destination::Image => self.image_builder.set_link(link),
            Destination::Item => self.item_builder.set_link(link),
        }
        Ok(())
    }
    pub fn set_description(
        &mut self,
        destination: &Destination,
        description: &str,
    ) -> Result<(), Error> {
        match destination {
            Destination::Channel => self.channel_builder.set_description(description),
            Destination::Image => return Err(Error::InvalidDestination),
            Destination::Item => self.item_builder.set_description(description),
        }
        Ok(())
    }
    pub fn set_language(&mut self, destination: &Destination, language: &str) -> Result<(), Error> {
        match destination {
            Destination::Channel => self.channel_builder.set_language(language),
            _ => return Err(Error::InvalidDestination),
        }
        Ok(())
    }
    pub fn set_copyright(
        &mut self,
        destination: &Destination,
        copyright: &str,
    ) -> Result<(), Error> {
        match destination {
            Destination::Channel => self.channel_builder.set_copyright(copyright),
            _ => return Err(Error::InvalidDestination),
        }
        Ok(())
    }
    pub fn set_pub_date(&mut self, destination: &Destination, pub_date: &str) -> Result<(), Error> {
        let pub_date = TryFrom::<DateTime<FixedOffset>>::get(&pub_date)?;
        match destination {
            Destination::Channel => self.channel_builder.set_pub_date(pub_date),
            Destination::Image => return Err(Error::InvalidDestination),
            Destination::Item => self.item_builder.set_pub_date(pub_date),
        }
        Ok(())
    }
    pub fn set_image(&mut self) {
        let image = self.image_builder.build();
        self.channel_builder.set_image(image);
        self.image_builder = ImageBuilder::default();
    }
    pub fn set_item(&mut self) {
        let item = self.item_builder.build();
        self.channel_builder.set_items(item);
        self.item_builder = ItemBuilder::default();
    }
    pub fn set_url(&mut self, destination: &Destination, url: &str) -> Result<(), Error> {
        let url = TryFrom::<Url>::get(&url)?;
        match destination {
            Destination::Channel => return Err(Error::InvalidDestination),
            Destination::Image => self.image_builder.set_url(url),
            Destination::Item => return Err(Error::InvalidDestination),
        }
        Ok(())
    }
    pub fn set_guid(&mut self, destination: &Destination, guid: &str) -> Result<(), Error> {
        let guid = TryFrom::<Url>::get(&guid)?;
        match destination {
            Destination::Channel => return Err(Error::InvalidDestination),
            Destination::Image => return Err(Error::InvalidDestination),
            Destination::Item => self.item_builder.set_guid(guid),
        }
        Ok(())
    }
    pub fn get(&mut self) -> Channel {
        self.channel_builder.build()
    }
}

trait TryFrom<T> {
    fn get(&self) -> Result<T, Error>;
}

impl TryFrom<DateTime<FixedOffset>> for &str {
    fn get(&self) -> Result<DateTime<FixedOffset>, Error> {
        let result = DateTime::parse_from_rfc2822(self)?;
        Ok(result)
    }
}

impl TryFrom<Url> for &str {
    fn get(&self) -> Result<Url, Error> {
        let url = Url::parse(self)?;
        Ok(url)
    }
}
#[derive(PartialEq)]
enum State {
    Initial,
    Rss,
    Title,
    Link,
    Description,
    Language,
    Copyright,
    PubDate,
    Guid,
    Image,
    Url,
    Item,
    Complete,
}

pub fn parse_document(body: &str) -> Result<Channel, Error> {
    let mut reader = Reader::from_str(body);
    let mut buffer = Vec::new();
    let mut text_buffer = String::new();
    let mut state = State::Initial;
    let mut target = Destination::Channel;

    let mut proxy = Proxy::new();

    while state != State::Complete {
        match reader.read_event(&mut buffer) {
            Ok(Event::Start(ref e)) => match e.name() {
                RSS_EL => state = State::Rss,
                TITLE_EL => state = State::Title,
                LINK_EL => state = State::Link,
                DESCRIPTION_EL => state = State::Description,
                LANGUAGE_EL => state = State::Language,
                COPYRIGHT_EL => state = State::Copyright,
                PUBDATE_EL => state = State::PubDate,
                IMAGE_EL => {
                    state = State::Image;
                    target = Destination::Image;
                }
                ITEM_EL => {
                    state = State::Item;
                    target = Destination::Item;
                }
                GUID_EL => state = State::Guid,
                URL_EL => state = State::Url,
                _ => (),
            },
            Ok(Event::Text(e)) => {
                text_buffer = match e.unescape_and_decode(&reader) {
                    Ok(text_buffer) => text_buffer,
                    Err(_) => return Err(Error::ParseLibrary),
                };
            }
            Ok(Event::End(ref e)) => match e.name() {
                RSS_EL => state = State::Complete,
                TITLE_EL => proxy.set_title(&target, &text_buffer),
                LINK_EL => proxy.set_link(&target, &text_buffer)?,
                DESCRIPTION_EL => proxy.set_description(&target, &text_buffer)?,
                LANGUAGE_EL => proxy.set_language(&target, &text_buffer)?,
                COPYRIGHT_EL => proxy.set_copyright(&target, &text_buffer)?,
                PUBDATE_EL => proxy.set_pub_date(&target, &text_buffer)?,
                IMAGE_EL => {
                    proxy.set_image();
                    target = Destination::Channel;
                }
                ITEM_EL => {
                    proxy.set_item();
                    target = Destination::Channel;
                }
                GUID_EL => proxy.set_guid(&target, &text_buffer)?,
                URL_EL => proxy.set_url(&target, &text_buffer)?,
                _ => (),
            },
            Ok(Event::Eof) => (),
            Err(e) => {
                eprintln!("Error at position {}: {:?}", reader.buffer_position(), e);
                return Err(Error::ParseLibrary);
            }
            _ => (),
        }
    }
    Ok(proxy.get())
}
