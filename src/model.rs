use crate::error::Error;
use crate::parser::Item;

use std::fmt;

pub struct Forecast {
    days: Vec<Daily>,
}

impl Default for Forecast {
    fn default() -> Self {
        Forecast { days: Vec::new() }
    }
}

impl Forecast {
    pub fn parse_from_items(items: &Vec<Item>) -> Result<Forecast, Error> {
        let mut days: Vec<Daily> = Vec::new();
        for item in items {
            days.push(Daily::parse_from_item_title_and_description(
                item.get_title(),
                item.get_description(),
            )?);
        }
        Ok(Forecast { days })
    }
    pub fn one(&self) -> Result<&Daily, Error> {
        if self.days.is_empty() {
            return Err(Error::Conversion);
        }
        Ok(&self.days[0])
    }
    pub fn two(&self) -> Result<&Daily, Error> {
        if self.days.is_empty() || self.days.len() != 3 {
            return Err(Error::Conversion);
        }
        Ok(&self.days[1])
    }
    pub fn three(&self) -> Result<&Daily, Error> {
        if self.days.is_empty() || self.days.len() != 3 {
            return Err(Error::Conversion);
        }
        Ok(&self.days[2])
    }
}

// move iterator
impl IntoIterator for Forecast {
    type Item = Daily;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.days.into_iter()
    }
}

// ref iterator
impl<'a> IntoIterator for &'a Forecast {
    type Item = &'a Daily;
    type IntoIter = std::slice::Iter<'a, Daily>;

    fn into_iter(self) -> Self::IntoIter {
        self.days.iter()
    }
}

// mut ref iterator
impl<'a> IntoIterator for &'a mut Forecast {
    type Item = &'a mut Daily;
    type IntoIter = std::slice::IterMut<'a, Daily>;

    fn into_iter(self) -> Self::IntoIter {
        self.days.iter_mut()
    }
}

pub struct Daily {
    summary: Summary,
    details: Details,
}

impl Default for Daily {
    fn default() -> Self {
        Daily {
            summary: Summary::default(),
            details: Details::default(),
        }
    }
}

impl fmt::Display for Daily {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t{}",
            self.summary.summary, self.details.temperature_max
        )
    }
}

impl Daily {
    pub fn parse_from_item_title_and_description(
        title: &str,
        description: &str,
    ) -> Result<Daily, Error> {
        Ok(Daily {
            summary: Summary::parse_from_str(title)?,
            details: Details::parse_from_str(description)?,
        })
    }
    pub fn summary(&self) -> &Summary {
        &self.summary
    }
    pub fn details(&self) -> &Details {
        &self.details
    }
}

pub struct Summary {
    summary: String,
}

impl Default for Summary {
    fn default() -> Self {
        Summary {
            summary: String::new(),
        }
    }
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Summary: {}", self.summary)
    }
}

impl Summary {
    pub fn parse_from_str(title: &str) -> Result<Summary, Error> {
        if title.is_empty() || title.len() > 1024 {
            return Err(Error::Conversion);
        }
        let fields: Vec<&str> = title.split(',').collect();
        if fields.is_empty() || fields.len() != 2 {
            return Err(Error::Conversion);
        }
        let (_, summary) = match fields[0].find(':') {
            Some(index) => ((), &fields[0][index..]),
            None => return Err(Error::Conversion),
        };
        if summary.len() <= 2 {
            return Err(Error::Conversion);
        }
        Ok(Summary {
            summary: String::from(&summary[2..]),
        })
    }
    pub fn summary(&self) -> &str {
        &self.summary
    }
}

pub struct Details {
    temperature_max: String,
    temperature_min: String,
    wind_direction: String,
    wind_speed: String,
    visibility: String,
    pressure: String,
    humidity: String,
    uv_risk: String,
    pollution_level: String,
    sunrise_time: String,
    sunset_time: String,
}

impl Default for Details {
    fn default() -> Self {
        Details {
            temperature_max: String::new(),
            temperature_min: String::new(),
            wind_direction: String::new(),
            wind_speed: String::new(),
            visibility: String::new(),
            pressure: String::new(),
            humidity: String::new(),
            uv_risk: String::new(),
            pollution_level: String::new(),
            sunrise_time: String::new(),
            sunset_time: String::new(),
        }
    }
}

impl fmt::Display for Details {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "Maximum Temperature: {}\nMinimum Temperature: {}\nWind Direction: {}\nWind Speed: {}\nVisibility: {}\nPressure: {}\nHumidity: {}\nUV Risk: {}\nPollution: {}\nSunrise: {}\nSunset: {}\n",
            self.temperature_max,
            self.temperature_min,
            self.wind_direction,
            self.wind_speed,
            self.visibility,
            self.pressure,
            self.humidity,
            self.uv_risk,
            self.pollution_level,
            self.sunrise_time,
            self.sunset_time
        )
    }
}

impl Details {
    pub fn parse_from_str(description: &str) -> Result<Details, Error> {
        let mut details = Details::default();

        if description.is_empty() || description.len() > 4096 {
            return Err(Error::Conversion);
        }
        let fields: Vec<&str> = description.split(',').collect();
        if fields.is_empty() || fields.len() != 11 {
            return Err(Error::Conversion);
        }
        for field in fields {
            let (first, rest) = match field.find(':') {
                Some(index) => (&field[0..index], &field[index..]),
                None => return Err(Error::Conversion),
            };
            if rest.len() <= 2 {
                return Err(Error::Conversion);
            }
            let field = match first.trim() {
                "Maximum Temperature" => &mut details.temperature_max,
                "Minimum Temperature" => &mut details.temperature_min,
                "Wind Direction" => &mut details.wind_direction,
                "Wind Speed" => &mut details.wind_speed,
                "Visibility" => &mut details.visibility,
                "Pressure" => &mut details.pressure,
                "Humidity" => &mut details.humidity,
                "UV Risk" => &mut details.uv_risk,
                "Pollution" => &mut details.pollution_level,
                "Sunrise" => &mut details.sunrise_time,
                "Sunset" => &mut details.sunset_time,
                _ => return Err(Error::Conversion),
            };
            *field = String::from(&rest[2..]);
        }
        Ok(details)
    }
}
