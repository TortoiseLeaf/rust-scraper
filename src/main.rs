use reqwest::Client;
use scraper::{Html, Selector};

let client = Client::new();

let mut res = client.get("http://books.toscrape.com/")
.send()
.unwrap();

// a try-catch to stop if there's an error or no text
let body = res.text().unwrap();

let document = Html::parse_document(&body);

// looks for headers
let book_title_selector = Selector::parse("h3 > a").unwrap();

// <Vec<_>> is a dynamically sized array, a vector
for book_title in document.select(&book_title_selector) {
    let title = book_title.text().collect::<Vec<_>>();
    println!("Title: {}", title[0]);
}

let book_price_selector = Selector::parse(".price_color").unwrap();

for book_price in document.select(&book_price_selector) {
    let price = book_price.text().collect::<Vec<_>>();
    println!("Price: {}", price[0]);
}

fn main() {
    println!("Hello, world!");
}
