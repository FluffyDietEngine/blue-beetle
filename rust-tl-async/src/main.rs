extern crate reqwest;
extern crate tl;
use tokio;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let url = "https://en.wikipedia.org/wiki/Lists_of_books";
    let resp = reqwest::get(url).await.unwrap();
    let body = resp.text().await.unwrap();
    let start = Instant::now();
    for _ in 0..1000 {
        let dom = tl::parse(&body, tl::ParserOptions::default()).unwrap();
        let _anchors = dom.query_selector("a[href]").unwrap();
    }
    println!("Average time: {} ms", start.elapsed().as_millis() / 1000);
}