use crate::search::{Query, SearchResult};

use reqwest::get;
use url::Url;

pub async fn search(query: &Query) -> Vec<SearchResult> {
    let url = format!("https://lite.duckduckgo.com/lite?q={}", query.query);

    let response = get(url).await.unwrap();
    let body = response.text().await.unwrap();

    let mut results = Vec::new();

    let re = regex::Regex::new(r#"<tr>\s*<td valign="top">.*?\.&nbsp;</td>\s*<td>\s*<a rel="nofollow" href="(.*?)" class='result-link'>(.*?)</a>\s*</td>\s*</tr>\s*<tr>\s*<td>&nbsp;&nbsp;&nbsp;</td>\s*<td class='result-snippet'>\s*(.*?)\s*</td>\s*</tr>"#).unwrap();
    for capture in re.captures_iter(&body) {
        let url = {
            let duckduckgo_url = format!("https:{}", &capture[1]);
            let parsed_url = Url::parse(duckduckgo_url.as_str()).unwrap();
            parsed_url.query_pairs()
                .next().unwrap()
                .1.to_string()
        };
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
