use crate::search::{Query, SearchResult};

use reqwest::Client;
use regex::Regex;
use url::Url;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r#"<tr>\s*<td valign="top">.*?\.&nbsp;</td>\s*<td>\s*<a rel="nofollow" href="(.*?)" class='result-link'>(.*?)</a>\s*</td>\s*</tr>\s*<tr>\s*<td>&nbsp;&nbsp;&nbsp;</td>\s*<td class='result-snippet'>\s*(.*?)\s*</td>\s*</tr>"#).unwrap();
    static ref HTTP_CLIENT: Client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build().unwrap();
}

pub async fn search(query: &Query) -> Vec<SearchResult> {
    let url = format!("https://lite.duckduckgo.com/lite?q={}", query.query);
    let response = HTTP_CLIENT.get(&url).send().await.unwrap();
    let body = response.text().await.unwrap();

    let mut results = Vec::new();

    for capture in RE.captures_iter(&body) {
        let duckduckgo_url = format!("https:{}", &capture[1]);
        let parsed_url = Url::parse(&duckduckgo_url).unwrap();
        let url = parsed_url.query_pairs().next().unwrap().1.to_string();
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
