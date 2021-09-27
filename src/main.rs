use home::home_dir;
use rocket::response::Redirect;
use rocket::State;
use serde::Deserialize;
use std::env::{current_dir, var};
use std::path::Path;
use urlencoding::encode;

#[macro_use]
extern crate rocket;

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
    pub fn new(default: Shortcut) -> ShortcutRegistry {
        ShortcutRegistry {
            default,
            shortcuts: Vec::new(),
        }
    }

    pub fn add(&mut self, shortcut: Shortcut) {
        self.shortcuts.push(shortcut);
    }

    pub fn import_from_csv(&mut self, path: &String) -> Result<usize, String> {
        match csv::Reader::from_path(path) {
            Ok(mut reader) => {
                for result in reader.deserialize() {
                    match result {
                        Ok(shortcut) => self.add(shortcut),
                        Err(err) => return Err(format!("Failed to parse shortcut: {}", err)),
                    }
                }
            }
            Err(e) => return Err(format!("Failed to read shortcuts.csv: {}", e)),
        }

        Ok(self.shortcuts.len())
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

#[get("/<q>")]
fn query(q: &str, shortcuts: &State<ShortcutRegistry>) -> Redirect {
    let redirect_url = shortcuts.match_query(q);

    Redirect::to(redirect_url)
}

fn find_config_file() -> String {
    let path = String::from("/etc/localbangs.csv");
    if Path::new(&path).exists() {
        return path;
    }
    match var("XDG_CONFIG_HOME") {
        Ok(xdg_config_home) => {
            println!("{}", xdg_config_home);

            let path = format!("{}/localbang.csv", xdg_config_home);
            if Path::new(&path).exists() {
                return path;
            }
        }
        Err(_) => {}
    }

    match home_dir() {
        Some(home_dir) => {
            println!("{}", home_dir.display());

            let path = format!("{}/.localbang.csv", home_dir.display());
            if Path::new(&path).exists() {
                return path;
            }
        }
        None => {}
    };

    match current_dir() {
        Ok(current_dir) => {
            println!("{}", current_dir.display());
            let path = format!("{}/localbang.csv", current_dir.display());
            if Path::new(&path).exists() {
                return path;
            }
        }
        Err(_) => {}
    }

    panic!("Could not find localbangs.csv");
}

#[launch]
fn rocket() -> _ {
    let mut registry = ShortcutRegistry::new(Shortcut::new(
        String::from("google"),
        String::from("https://google.com/search?q=%s"),
    ));

    let config_path = find_config_file();
    match registry.import_from_csv(&config_path) {
        Ok(amount) => println!("Imported {} shortcuts from {}", amount, &config_path),
        Err(e) => println!("Failed to import shortcuts from {}: {}", &config_path, e),
    }

    rocket::build().manage(registry).mount("/", routes![query])
}
