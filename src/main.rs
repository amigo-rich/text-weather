use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: text_weather <url>");
        eprintln!("Only bbc rss feeds are currently supported");
        std::process::exit(1)
    }
    text_weather::run(&args[1]);
}
