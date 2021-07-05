use clap::{App, Arg};

fn main() {
    let matches = App::new("text_weather")
        .arg(
            Arg::with_name("segment")
                .long("segment")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let segment = matches.value_of("segment").unwrap();

    match text_weather::run(segment) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            panic!();
        }
    }
}
