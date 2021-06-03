use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
enum DescriptionKey {
    MaximumTemperature,
    MinimumTemperature,
    WindDirection,
    WindSpeed,
    Visibility,
    Pressure,
    Humidity,
    UVRisk,
    Pollution,
    Sunrise,
    Sunset,
}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

#[derive(Debug)]
pub struct DescriptionModel {
    model: HashMap<DescriptionKey, String>,
}

impl DescriptionModel {
    const EXPECTED_FIELDS: usize = 11;
    const DESCRIPTION_MAX: usize = 256;
    pub fn new(d: &str) -> Result<DescriptionModel, Error> {
        if d.is_empty() || d.len() > Self::DESCRIPTION_MAX {
            eprintln!("From d");
            return Err(Error::InvalidInput);
        }
        let fields: Vec<&str> = d.split(',').collect();
        if fields.is_empty() || fields.len() != Self::EXPECTED_FIELDS {
            eprintln!("Fields: {:?}", fields);
            return Err(Error::InvalidInput);
        }
        let mut model: HashMap<DescriptionKey, String> = HashMap::new();
        for field in fields {
            let kv: Vec<&str> = field.split(": ").collect();
            if kv.is_empty() || kv.len() != 2 {
                eprintln!("KV: {:?}", kv);
                return Err(Error::InvalidInput);
            }
            let desc_key = match kv[0].trim() {
                "Maximum Temperature" => DescriptionKey::MaximumTemperature,
                "Minimum Temperature" => DescriptionKey::MinimumTemperature,
                "Wind Direction" => DescriptionKey::WindDirection,
                "Wind Speed" => DescriptionKey::WindSpeed,
                "Visibility" => DescriptionKey::Visibility,
                "Pressure" => DescriptionKey::Pressure,
                "Humidity" => DescriptionKey::Humidity,
                "UV Risk" => DescriptionKey::UVRisk,
                "Pollution" => DescriptionKey::Pollution,
                "Sunrise" => DescriptionKey::Sunrise,
                "Sunset" => DescriptionKey::Sunset,
                &_ => {
                    eprintln!("KV0: '{}'", kv[0]);
                    panic!("Got: {}", kv[0]);
                }
            };
            model.insert(desc_key, kv[1].to_string());
        }
        Ok(DescriptionModel { model })
    }
}
