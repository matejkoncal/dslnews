use inquire::Select;
use reqwest::get;
use select::document::Document;
use select::predicate::{Attr, Name};
use std::fmt;

struct NewsItem {
    title: String,
    link: String,
}

impl fmt::Display for NewsItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_dsl_news("http://dsl.sk/").await;
    Ok(())
}

async fn load_dsl_news(url: &str) {
    let resp = get(url).await.unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp.text().await.unwrap().as_bytes()).unwrap();
    let news_box = document.find(Attr("id", "news_box")).next().unwrap();

    let news_items: Vec<NewsItem> = news_box
        .find(Name("a"))
        .filter(|x| x.text().chars().next().unwrap() != '(')
        .map(|x| NewsItem {
            title: x.text().to_string(),
            link: x.attr("href").unwrap().to_string(),
        })
        .collect();

    let ans = Select::new("Which news do you want to open in the browser?", news_items)
        .prompt()
        .unwrap();

    open::that("http://dsl.sk/".to_string() + &ans.link).unwrap();
}
