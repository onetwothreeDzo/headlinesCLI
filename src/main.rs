use std::error::Error;

use colour::{dark_green, yellow};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
    author: Option<String>,
    description: Option<String>,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        dark_green!("> {}\n", a.title);
        yellow!("{}\n", a.url);
        match &a.description {
            Some(description) => println!("~ {}\n\n", description),
            None => println!("---"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let apikey = std::env::args()
        .nth(1)
        .expect("your api key is needed. Following $cargo run -- <your-key>");
    let url = format!(
        "https://newsapi.org/v2/top-headlines?country=us&category=business&apiKey={}",
        apikey
    );
    let articles = get_articles(&url)?;
    render_articles(&articles);
    Ok(())
}
