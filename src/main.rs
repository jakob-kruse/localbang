use rocket::response::Redirect;
use serde::Deserialize;
use urlencoding::encode;

#[macro_use]
extern crate rocket;

#[derive(Debug, Deserialize)]
struct SearchShortcut {
    keyword: String,
    url: String,
}

impl SearchShortcut {
    fn query_matches(&self, query: &str) -> bool {
        query.contains(&self.keyword)
    }

    fn query(&self, query: &str) -> String {
        let query = query.replace(&self.keyword, "");
        let query = query.trim();
        let encoded_query = encode(&query);
        self.url.replace("%s", encoded_query.trim())
    }
}

fn read_shortcuts() -> Result<Vec<SearchShortcut>, String> {
    let mut reader = csv::Reader::from_path("shortcuts.csv").expect("Failed to read shortcuts.csv");

    let mut shortcuts = Vec::new();
    for result in reader.deserialize() {
        let shortcut: SearchShortcut = result.expect("Failed to parse shortcut");
        shortcuts.push(shortcut);
    }
    Ok(shortcuts)
}

fn find_shortcut(query: &str) -> Option<SearchShortcut> {
    let shortcuts = read_shortcuts().expect("Failed to read shortcuts");

    for shortcut in shortcuts {
        if shortcut.query_matches(query) {
            println!("Matching shortcut: {}", shortcut.keyword);
            return Some(shortcut);
        }
    }

    None
}

#[get("/<q>")]
fn query(q: &str) -> Redirect {
    let shortcut = find_shortcut(q);
    let redirect_url = match shortcut {
        Some(shortcut) => shortcut.query(q),
        None => format!("https://www.google.com/search?q={}", encode(q)),
    };
    Redirect::to(redirect_url)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![query])
}
