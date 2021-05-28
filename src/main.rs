const URL: &str = "https://weather-broker-cdn.api.bbci.co.uk/en/observation/rss/2638077";

fn main() {
    text_weather::run(URL);
}
