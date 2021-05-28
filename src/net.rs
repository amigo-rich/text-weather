use reqwest::blocking::get;
use url::Url;

pub enum Error {
    ReqwestGet,
    ReqwestBody,
}

pub fn reqwest_fetch_url(url: Url) -> Result<String, Error> {
    let response = match get(url) {
        Ok(response) => response,
        Err(_) => return Err(Error::ReqwestGet),
    };
    let text = match response.text() {
        Ok(text) => text,
        Err(_) => return Err(Error::ReqwestBody),
    };
    Ok(text)
}
