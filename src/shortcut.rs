use serde::Deserialize;
use urlencoding::encode;

use crate::config::Config;

#[derive(Debug, Deserialize)]
pub struct Shortcut {
    keyword: String,
    url: String,
}

impl Shortcut {
    pub fn new(keyword: String, url: String) -> Shortcut {
        Shortcut { keyword, url }
    }
}

pub struct ShortcutRegistry {
    search_engine: Shortcut,
    shortcuts: Vec<Shortcut>,
}

impl ShortcutRegistry {
    pub fn new(config: Config) -> Result<ShortcutRegistry, String> {
        let search_engine: Shortcut =
            Shortcut::new(String::from("def"), String::from(&config.search_engine));
        let mut shortcuts: Vec<Shortcut> = Vec::new();

        match &config.shortcuts_path {
            Some(path) => match csv::Reader::from_path(&path) {
                Ok(mut reader) => {
                    for result in reader.deserialize() {
                        match result {
                            Ok(shortcut) => shortcuts.push(shortcut),
                            Err(err) => return Err(format!("Failed to parse shortcut: {}", err)),
                        }
                    }
                }
                Err(_) => {
                    println!("Config file at \"{}\" not found.", &path)
                }
            },
            None => {
                println!("No shortcuts file specified in config. This will just be a search engine mirror.");
            }
        }

        Ok(ShortcutRegistry {
            search_engine,
            shortcuts,
        })
    }

    pub fn match_query(&self, query: &str) -> String {
        let mut matched_shortcut = None;
        for shortcut in &self.shortcuts {
            if query.contains(shortcut.keyword.as_str()) {
                matched_shortcut = Some(shortcut);
                break;
            }
        }

        let shortcut = matched_shortcut.unwrap_or(&self.search_engine);

        let clean_query = query.replace(shortcut.keyword.as_str(), "");
        let clean_query = clean_query.trim();

        shortcut
            .url
            .replace("%s", &encode(clean_query).into_owned())
    }
}
