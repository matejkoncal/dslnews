use inquire::Select;
use reqwest::get;
use select::document::Document;
use select::predicate::{Attr, Name};

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

    let titles: Vec<String> = news_box
        .find(Name("a"))
        .filter(|x| x.text().chars().next().unwrap() != '(')
        .map(|x| x.text())
        .collect();

    let urls: Vec<String> = news_box
        .find(Name("a"))
        .filter(|x| x.text().chars().next().unwrap() != '(')
        .map(|x| x.attr("href").unwrap().to_string())
        .collect();

    let ans = Select::new(
        "Which news do you want to open in the browser?",
        titles.clone(),
    )
    .prompt()
    .unwrap();

    let index_of_selected = titles.iter().position(|x| x == &ans).unwrap();
    open::that("http://dsl.sk/".to_string() + &urls[index_of_selected]).unwrap();
}
