use std::env::args;

mod path;

#[derive(Debug)]
pub struct Config {
    pub shortcuts_path: Option<String>,
    pub search_engine: String,
}

pub fn get_config() -> Config {
    let args = args().collect::<Vec<String>>();

    if args.contains(&String::from("-h")) {
        println!("Usage: localbang <shortcuts_file> <search_engine>");
        std::process::exit(0);
    }

    let shortcuts_path = path::find_shortcuts_path(&args);
    let search_engine = match args.get(2) {
        Some(arg) => arg.clone(),
        None => String::from("https://www.google.com/search?q=%s"),
    };

    if !search_engine.contains("%s") {
        eprintln!("Search engine must contain %s.");
        std::process::exit(1);
    }

    let config = Config {
        shortcuts_path,
        search_engine,
    };

    config
}
