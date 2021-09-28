use rocket::response::Redirect;
use rocket::State;

#[macro_use]
extern crate rocket;

mod config;
mod shortcut;

#[get("/<q>")]
fn query(q: &str, shortcuts: &State<shortcut::ShortcutRegistry>) -> Redirect {
    let redirect_url = shortcuts.match_query(q);

    Redirect::to(redirect_url)
}

#[launch]
fn rocket() -> _ {
    let app_config = config::get_config();

    let registry = match shortcut::ShortcutRegistry::new(app_config) {
        Ok(registry) => registry,
        Err(e) => {
            eprintln!("{}.", e);
            std::process::exit(1);
        }
    };

    rocket::build().manage(registry).mount("/", routes![query])
}
