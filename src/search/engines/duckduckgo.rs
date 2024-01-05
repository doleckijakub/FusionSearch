use crate::search::{Query, SearchResult};

use reqwest::{Client, RequestBuilder};
use regex::Regex;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r#"<tr>\s*<td valign="top">.*?\.&nbsp;</td>\s*<td>\s*<a rel="nofollow" href="(.*?)" class='result-link'>(.*?)</a>\s*</td>\s*</tr>\s*<tr>\s*<td>&nbsp;&nbsp;&nbsp;</td>\s*<td class='result-snippet'>\s*(.*?)\s*</td>\s*</tr>"#).unwrap();
}

pub fn request(request: Client, query: &Query) -> RequestBuilder {
    request
        .get(format!("https://lite.duckduckgo.com/lite?q={}", query.query))
}

pub fn search_results(response: &String) -> Vec<SearchResult> {
    let mut results = Vec::new();

    for capture in RE.captures_iter(&response) {
        let url = capture[1].to_string();
        let title = capture[2].to_string();
        let snippet = capture[3].to_string();

        results.push(SearchResult {
            url,
            title,
            snippet
        });
    }

    results
}
