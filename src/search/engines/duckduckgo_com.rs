use crate::search::{Query, SearchResult};

use reqwest::{Client, RequestBuilder};
use scraper::{Selector, Html};

lazy_static::lazy_static! {
    static ref RESULT_TABLE_SELECTOR: Selector = Selector::parse(r#"html > body > form > div.filters > table"#).unwrap();
    static ref TR_ROWS_SELECTOR:      Selector = Selector::parse("tr:not(:last-child)").unwrap();
    static ref A_TAG_SELECTOR:        Selector = Selector::parse("td a.result-link").unwrap();
    static ref TD_CONTENT_SELECTOR:   Selector = Selector::parse("td.result-snippet").unwrap();
}

pub const URL: &str = "duckduckgo.com";

pub fn request(request: Client, query: &Query) -> RequestBuilder {
    request
        .get(format!("https://lite.duckduckgo.com/lite?q={}", query.query))
}

pub fn search_results(response: &String) -> Vec<SearchResult> {
    let mut results = Vec::new();

    let html = Html::parse_document(response);

    let result_table = html.select(&RESULT_TABLE_SELECTOR).last().unwrap();

    let tr_rows = result_table.select(&TR_ROWS_SELECTOR).collect::<Vec<_>>();

    let mut offset = 0;
    while tr_rows.len() >= offset + 4 {
        let tr_title = &tr_rows[offset];
        let tr_content = &tr_rows[offset + 1];
        offset += 4;

        if tr_content.value().attr("class").unwrap_or("") == "result-sponsored" {
            continue;
        }

        let a_tag = tr_title.select(&A_TAG_SELECTOR).next();
        if let Some(a_tag) = a_tag {
            let td_content = tr_content.select(&TD_CONTENT_SELECTOR).next();

            if let Some(td_content) = td_content {
                let url = a_tag.value().attr("href").unwrap().to_string();
                let title = a_tag.text().collect();
                let snippet = td_content.text().collect();

                results.push(SearchResult { url, title, snippet });
            }
        }
    }

    results
}
