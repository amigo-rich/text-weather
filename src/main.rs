use text_weather::dayoption::Day;

use clap::{App, Arg};

fn main() {
    let matches = App::new("text_weather")
        .arg(
            Arg::with_name("segment")
                .long("segment")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("today")
                .long("today")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("tomorrow")
                .long("tomorrow")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("overmorrow")
                .long("overmorrow")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    let segment = matches.value_of("segment").unwrap();
    let mut day_option: Option<Day> = None;
    if matches.is_present("today") {
        day_option.replace(Day::Today);
    } else if matches.is_present("tomorrow") {
        day_option.replace(Day::Tomorrow);
    } else if matches.is_present("overmorrow") {
        day_option.replace(Day::Overmorrow);
    }
    match text_weather::run(segment, day_option) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            panic!();
        }
    }
}
