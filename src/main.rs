use clap::Parser;
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
/// Simple program for reading news on dsl.sk
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Open in browser or in terminal
    #[clap(short, long)]
    open_browser: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let open_in_browser = args.open_browser;
    load_dsl_news("http://dsl.sk/", open_in_browser).await;
    Ok(())
}

async fn load_dsl_news(url: &str, open_in_broeser: bool) {
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

    if open_in_broeser {
        show_in_browser(ans);
    } else {
        show_in_terminal(&ans).await;
    }
}

fn show_in_browser(ans: NewsItem) {
    open::that("http://dsl.sk/".to_string() + &ans.link).unwrap();
}

async fn show_in_terminal(ans: &NewsItem) {
    let resp_news = get("http://dsl.sk/".to_string() + &ans.link).await.unwrap();
    assert!(resp_news.status().is_success());
    let document_news = Document::from_read(resp_news.text().await.unwrap().as_bytes()).unwrap();
    print!(
        "{}",
        document_news
            .find(Attr("class", "article_body"))
            .next()
            .unwrap()
            .text()
    );
}
