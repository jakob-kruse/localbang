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
    default: Shortcut,
    shortcuts: Vec<Shortcut>,
}

impl ShortcutRegistry {
    pub fn new(config: Config) -> Result<ShortcutRegistry, String> {
        let default: Shortcut = Shortcut::new(
            String::from("def"),
            String::from(&config.default_search_engine),
        );
        let mut shortcuts: Vec<Shortcut> = Vec::new();
        match csv::Reader::from_path(&config.shortcuts_path) {
            Ok(mut reader) => {
                for result in reader.deserialize() {
                    match result {
                        Ok(shortcut) => shortcuts.push(shortcut),
                        Err(err) => return Err(format!("Failed to parse shortcut: {}", err)),
                    }
                }
            }
            Err(e) => return Err(format!("Failed to read shortcuts.csv: {}", e)),
        }

        Ok(ShortcutRegistry { default, shortcuts })
    }

    pub fn match_query(&self, query: &str) -> String {
        let mut matched_shortcut = None;
        for shortcut in &self.shortcuts {
            if query.contains(shortcut.keyword.as_str()) {
                matched_shortcut = Some(shortcut);
                break;
            }
        }

        let shortcut = matched_shortcut.unwrap_or(&self.default);

        let clean_query = query.replace(shortcut.keyword.as_str(), "");
        let clean_query = clean_query.trim();

        return shortcut
            .url
            .replace("%s", &encode(clean_query).into_owned());
    }
}
