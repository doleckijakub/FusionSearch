
use crate::search::{Query, SearchResult};

use reqwest::{Client, RequestBuilder};
use scraper::{Selector, Html, Element};

lazy_static::lazy_static! {
    static ref ANSWER_LIST_SELECTOR: Selector = Selector::parse("div.LGOjhe").unwrap();
    static ref RESULTS_SELECTOR:     Selector = Selector::parse("div[jscontroller=SC7lYd]").unwrap();
    static ref URL_SELECTOR:         Selector = Selector::parse("a").unwrap();
    static ref TITLE_SELECTOR:       Selector = Selector::parse("a > h3").unwrap();
    static ref SNIPPET_SELECTOR:     Selector = Selector::parse("div[data-sncf]").unwrap();
}

pub const URL: &str = "google.com";

pub fn request(request: Client, query: &Query) -> RequestBuilder {
    request
        .get(format!("https://google.com/search?q={}", query.query))
}

pub fn search_results(response: &String) -> Vec<SearchResult> {
    let mut results = Vec::new();
    
    let html = Html::parse_document(response);

    for answer in html.select(&ANSWER_LIST_SELECTOR) {
        let answer_snippet = answer.text().collect::<String>().trim().to_string();
        let url = answer
            .parent_element().unwrap()
            .parent_element().unwrap()
            .parent_element().unwrap()
            .select(&URL_SELECTOR)
            .next()
            .and_then(|a| a.value().attr("href").map(|href| href.to_string()));

        println!("answer: {} {}", url.unwrap_or(String::from("")), answer_snippet);
    }

    for result in html.select(&RESULTS_SELECTOR) {
        let url = result.select(&URL_SELECTOR)
            .next().unwrap()
            .attr("href").unwrap().to_string();

        let title = result.select(&TITLE_SELECTOR)
            .next().unwrap()
            .text().collect::<String>();

        let snippet = {
            let mut el_text = String::new();

            for el in result.select(&SNIPPET_SELECTOR) {
                el_text = el.text().collect::<String>();

                if el_text.len() != 0 {
                    break;
                }
            }

            el_text
        };
        
        results.push(SearchResult { url, title, snippet });
    }

    results
}
