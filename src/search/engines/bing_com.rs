use crate::search::{Query, SearchResult};

use reqwest::{Client, RequestBuilder};
use scraper::{Selector, Html};

lazy_static::lazy_static! {
    static ref RESULT_SELECTOR:  Selector = Selector::parse("#b_results > li.b_algo").unwrap();
    static ref LINK_SELECTOR:    Selector = Selector::parse("h2 > a").unwrap();
    static ref SNIPPET_SELECTOR: Selector = Selector::parse("p").unwrap();
    static ref SPAN_SELECTOR:    Selector = Selector::parse("span.algoSlug_icon").unwrap();
}

use url::Url;
use std::collections::HashMap;
use base64::{Engine, engine::general_purpose::URL_SAFE};

pub const URL: &str = "bing.com";

pub fn request(request: Client, query: &Query) -> RequestBuilder {
    request
        .get(format!("https://bing.com/search?q={}", query.query))
}

pub fn search_results(response: &String) -> Vec<SearchResult> {
    let mut results = Vec::new();

    let html = Html::parse_document(response);
    for result in html.select(&RESULT_SELECTOR) {
        if let Some(link) = result.select(&LINK_SELECTOR).next() {
            let mut url = link.value().attr("href").unwrap_or("").to_string();

            if url.starts_with("https://www.bing.com/ck/a?") {
                let url_query = Url::parse(&url).unwrap().query().unwrap().to_string();
                let parsed_url_query: HashMap<_, _> = url_query
                    .split('&')
                    .filter_map(|s| {
                        let mut parts = s.split('=');
                        Some((parts.next()?.to_string(), parts.next()?.to_string()))
                    })
                    .collect();

                if let Some(param_u) = parsed_url_query.get("u") {
                    let encoded_url = &param_u[2..];

                    let encoded_url = format!("{}{}",
                        encoded_url,
                        match encoded_url.len() % 4 {
                            0 => "",
                            1 => "===",
                            2 => "==",
                            3 => "=",
                            n => panic!("Unhandled case: {}", n)
                        }
                    );

                    if let Ok(decoded_url) = URL_SAFE.decode(encoded_url) {
                        if let Ok(decoded_str) = String::from_utf8(decoded_url) {
                            url = decoded_str;
                        }
                    }
                }
            }

            let title: String = link.text().collect();
            let mut snippet = String::new();

            for p in result.select(&SNIPPET_SELECTOR) {
                snippet.push_str(&p.text().collect::<String>());
            }

            results.push(SearchResult { url, title, snippet });
        }
    }

    results
}
