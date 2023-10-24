extern crate reqwest;
extern crate tl;
use std::time::Instant;

fn main() {
    let url = "https://en.wikipedia.org/wiki/Lists_of_books";
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).send().unwrap();
    let body = resp.text().unwrap();
    let start = Instant::now();
    for _ in 0..1000 {
        let dom = tl::parse(&body, tl::ParserOptions::default()).unwrap();
        let _anchors = dom.query_selector("a[href]").unwrap();
    }
    println!("Average time: {} ms", start.elapsed().as_millis() / 1000);
}
