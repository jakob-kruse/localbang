use std::env::args;

mod path;

pub struct Config {
    pub shortcuts_path: String,
    pub default_search_engine: String,
}

pub fn get_config() -> Config {
    let args = args().collect::<Vec<String>>();
    let shortcuts_path = path::find_shortcuts_path(&args);
    let default_search_engine = match args.get(2) {
        Some(arg) => arg.clone(),
        None => String::from("https://www.google.com/search?q=%s"),
    };

    Config {
        shortcuts_path,
        default_search_engine,
    }
}
