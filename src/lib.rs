mod error;
mod model;
use model::Forecast;
mod net;
use net::reqwest_fetch_url;
mod parser;
use parser::parse_document;

use std::io::{stdin, stdout};
use std::thread;
use std::time::Duration;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Row, Table, Tabs},
    Terminal,
};

use url::Url;

pub fn run(uri: &str) {
    let url = match Url::parse(uri) {
        Ok(url) => url,
        Err(_) => panic!("Invalid url: {}", uri),
    };

    let rss_body = match reqwest_fetch_url(url) {
        Ok(rss_body) => rss_body,
        Err(_) => panic!("Network error"),
    };

    let parsed = match parse_document(rss_body.as_str()) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!();
        }
    };
    let forecast = Forecast::parse_from_items(parsed.get_items()).unwrap();
    gui(&forecast);
}

pub fn forecast_to_table(f: &Forecast) -> Vec<Table> {
    vec![
        Table::new(vec![
            Row::new(vec!["Summary", f.one().unwrap().summary().summary()]),
            Row::new(vec![
                "Maximum temperature",
                f.one().unwrap().details().temperature_max(),
            ]),
            Row::new(vec![
                "Minimum temperature",
                f.one().unwrap().details().temperature_min(),
            ]),
            Row::new(vec![
                "Wind direction",
                f.one().unwrap().details().wind_direction(),
            ]),
            Row::new(vec!["Wind speed", f.one().unwrap().details().wind_speed()]),
            Row::new(vec!["Visibility", f.one().unwrap().details().visibility()]),
            Row::new(vec!["Pressure", f.one().unwrap().details().pressure()]),
            Row::new(vec!["Humidity", f.one().unwrap().details().humidity()]),
            Row::new(vec!["UV risk", f.one().unwrap().details().uv_risk()]),
            Row::new(vec!["Pollution", f.one().unwrap().details().pollution()]),
            Row::new(vec!["Sunrise", f.one().unwrap().details().sunrise()]),
            Row::new(vec!["Sunset", f.one().unwrap().details().sunset()]),
        ])
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .widths([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()),
        Table::new(vec![
            Row::new(vec!["Summary", f.two().unwrap().summary().summary()]),
            Row::new(vec![
                "Maximum temperature",
                f.two().unwrap().details().temperature_max(),
            ]),
            Row::new(vec![
                "Minimum temperature",
                f.two().unwrap().details().temperature_min(),
            ]),
            Row::new(vec![
                "Wind direction",
                f.two().unwrap().details().wind_direction(),
            ]),
            Row::new(vec!["Wind speed", f.two().unwrap().details().wind_speed()]),
            Row::new(vec!["Visibility", f.two().unwrap().details().visibility()]),
            Row::new(vec!["Pressure", f.two().unwrap().details().pressure()]),
            Row::new(vec!["Humidity", f.two().unwrap().details().humidity()]),
            Row::new(vec!["UV risk", f.two().unwrap().details().uv_risk()]),
            Row::new(vec!["Pollution", f.two().unwrap().details().pollution()]),
            Row::new(vec!["Sunrise", f.two().unwrap().details().sunrise()]),
            Row::new(vec!["Sunset", f.two().unwrap().details().sunset()]),
        ])
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .widths([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()),
        Table::new(vec![
            Row::new(vec!["Summary", f.three().unwrap().summary().summary()]),
            Row::new(vec![
                "Maximum temperature",
                f.three().unwrap().details().temperature_max(),
            ]),
            Row::new(vec![
                "Minimum temperature",
                f.three().unwrap().details().temperature_min(),
            ]),
            Row::new(vec![
                "Wind direction",
                f.three().unwrap().details().wind_direction(),
            ]),
            Row::new(vec![
                "Wind speed",
                f.three().unwrap().details().wind_speed(),
            ]),
            Row::new(vec![
                "Visibility",
                f.three().unwrap().details().visibility(),
            ]),
            Row::new(vec!["Pressure", f.three().unwrap().details().pressure()]),
            Row::new(vec!["Humidity", f.three().unwrap().details().humidity()]),
            Row::new(vec!["UV risk", f.three().unwrap().details().uv_risk()]),
            Row::new(vec!["Pollution", f.three().unwrap().details().pollution()]),
            Row::new(vec!["Sunrise", f.three().unwrap().details().sunrise()]),
            Row::new(vec!["Sunset", f.three().unwrap().details().sunset()]),
        ])
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .widths([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()),
    ]
}

pub fn gui(f: &Forecast) {
    let stdout = stdout().into_raw_mode().unwrap();
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut index = 0;
    let forecast_tables = forecast_to_table(&f);
    let day_one = &forecast_tables[0];
    let day_two = &forecast_tables[1];
    let day_three = &forecast_tables[2];
    loop {
        let stdin = stdin();
        /* tui */
        terminal
            .draw(|f| {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title("Weather forecast - data from the BBC")
                    .border_type(BorderType::Rounded);
                f.render_widget(block, f.size());
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(33),
                            Constraint::Percentage(34),
                            Constraint::Percentage(33),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                f.render_widget(day_one.clone(), chunks[0]);
                f.render_widget(day_two.clone(), chunks[1]);
                f.render_widget(day_three.clone(), chunks[2]);
                //                f.render_widget(day_one_table, left);
                /*
                let screen = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(5), Constraint::Percentage(95)].as_ref())
                    .split(f.size());
                let panels = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split();
                let titles: Vec<Spans> = tabs
                    .iter()
                    .map(|t| {
                        let (first, rest) = t.split_at(1);
                        Spans::from(vec![
                            Span::styled(first, Style::default().fg(Color::Blue)),
                            Span::styled(rest, Style::default().fg(Color::Green)),
                        ])
                    })
                    .collect();
                let tabs = Tabs::new(titles)
                    .block(Block::default().title("Weather Forecast"))
                    .select(index)
                    .style(Style::default().fg(Color::Cyan))
                    .highlight_style(
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .bg(Color::White),
                    );
                f.render_widget(tabs, chunks[0]);
                let block = Block::default()
                    .title("Summary")
                    .style(Style::default().bg(Color::LightYellow));
                f.render_widget(block, chunks[1]);
                */
                /*
                let blockb = Block::default()
                    .title("Details")
                    .style(Style::default().bg(Color::Magenta));
                f.render_widget(blockb, chunks[1]);
                */
                /*
                let inner = match index {
                    0 => Block::default().title("Day 1").borders(Borders::ALL),
                    1 => Block::default().title("Day 2").borders(Borders::ALL),
                    2 => Block::default().title("Day 3").borders(Borders::ALL),
                    _ => unreachable!(),
                };
                f.render_widget(inner, size);
                */
            })
            .unwrap();

        /* key input */
        let ev = stdin.events().next().unwrap();
        match ev.unwrap() {
            Event::Key(k) => match k {
                Key::Char('q') => return,
                Key::Left => {
                    if index == 0 {
                        index = 2;
                    } else {
                        index = index - 1;
                    }
                }
                Key::Right => {
                    if index == 2 {
                        index = 0;
                    } else {
                        index = index + 1;
                    }
                }
                _ => (),
            },
            _ => break,
        }
        thread::sleep(Duration::from_millis(50));
    }
}
