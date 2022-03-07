extern crate reqwest;

use select::document::Document;
use select::predicate::{Attr, Name};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_dsl_news("http://dsl.sk/").await;
    Ok(())
}

async fn load_dsl_news(url: &str) {
    let resp = reqwest::get(url).await.unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp.text().await.unwrap().as_bytes()).unwrap();
    let news_box = document.find(Attr("id", "news_box")).next().unwrap();
    for new_item in news_box
        .find(Name("a"))
        .filter(|x| x.text().chars().next().unwrap() != '(')
    {
        let text = new_item.text();
        println!("{}", text);
    }
}
