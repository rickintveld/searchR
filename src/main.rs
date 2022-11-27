#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Search engine addon"
}

#[get("/search?<key>")]
fn search(key: String) -> Redirect {
    let command: &str = utils::get_command_from_query_string(&key);
    let redirect_url: String = match command {
        "tw" => utils::twitter::construct_twitter_url(&key),
        "gh" => utils::github::construct_github_url(&key),
        "tv" => utils::tradingview::construct_tradingview_url(&key),
        "sc" => utils::soundcloud::construct_soundcloud_url(&key),
        _ => utils::google::construct_google_search_url(&key),
    };

    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}
