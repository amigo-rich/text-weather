use crate::error::Error;

use reqwest::blocking::get;
use url::Url;

pub fn reqwest_fetch_url(url: Url) -> Result<String, Error> {
    let response = get(url)?;
    let text = response.text()?;
    Ok(text)
}
