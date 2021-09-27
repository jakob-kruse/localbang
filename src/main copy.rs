use rocket::response::Redirect;
use serde::Deserialize;
use urlencoding::encode;

#[macro_use]
extern crate rocket;

#[derive(Clone)]
struct SearchUrl {
    url: String,
}

impl SearchUrl {
    fn new(url: String) -> SearchUrl {
        if !url.contains("%s") {
            panic!("URL must contain %s");
        }
        SearchUrl { url }
    }
    fn query(&self, query: &str) -> String {
        let encoded_query = encode(query);
        self.url.replace("%s", &encoded_query.into_owned())
    }
}

#[derive(Debug, Deserialize)]
struct ShortCut {
    key: String,
    url: String,
}

impl ShortCut {
    fn matches(&self, search: &str) -> bool {
        search.contains(&self.key)
    }
}

fn get_search_url(query: &str) -> SearchUrl {
    let default_search_url = SearchUrl::new(String::from("https://google.com/search?q=%s"));

    let shortcuts = match read_searchcuts() {
        Ok(shortcuts) => shortcuts,
        Err(err) => {
            println!("Failed reading shortcuts: {}", err);
            vec![]
        }
    };
    println!("{}", shortcuts.len());
    for shortcut in shortcuts.iter() {
        if shortcut.matches(query) {
            return shortcut.url.clone();
        }
    }

    println!("No shortcut found, using default search url");

    default_search_url
}

fn read_searchcuts() -> Result<Vec<ShortCut>, std::io::Error> {
    Ok(vec![])
}

#[get("/<q>")]
fn query(q: &str) -> Redirect {
    let search_url = get_search_url(q);
    let redirect_url = search_url.query(q);
    println!("Redirecting to {}", redirect_url);
    Redirect::to(redirect_url)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![query])
}
