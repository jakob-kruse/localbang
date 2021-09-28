use std::process;

use clap::{App, Arg, crate_authors, crate_description, crate_version};

mod path;

#[derive(Debug)]

pub struct Config {
    pub shortcuts_path: String,
    pub search_engine: String,
    pub host: String,
    pub port: u16
}

pub fn parse_args() -> Config {
    let matches = App::new("localbang")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("shortcuts_path").short("s").long("shortcuts").takes_value(true).help("the path to the shortcut csv file"))
        .arg(Arg::with_name("search_engine").short("e").long("search").takes_value(true).default_value("https://google.com/search?q=%s").help("the url of the default saerch engine to use when no bang matches"))
        .arg(Arg::with_name("host").short("h").long("host").takes_value(true).default_value("0.0.0.0").help("the host to bind to"))
        .arg(Arg::with_name("port").short("p").long("port").takes_value(true).default_value("3000").help("the port to bind to"))
        .about(crate_description!()).get_matches();

    let shortcuts_path = match matches.value_of("shortcuts_path") {
        Some(path) => String::from(path),
        None => match path::find_shortcuts_path() {
            Some(path) => String::from(path),
            None => {
                println!("Could not find shortcuts file");
                process::exit(1);
            }
        }
    };

    Config {
        shortcuts_path,
        host: String::from(matches.value_of("host").unwrap()),
        port: matches.value_of("port").unwrap().parse::<u16>().unwrap(),
        search_engine: String::from(matches.value_of("search_engine").unwrap()),
    }
}