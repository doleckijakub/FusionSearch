use crate::search::{Query, SearchResult, Error, http_get_text};

use regex::Regex;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r#"<tr>\s*<td valign="top">.*?\.&nbsp;</td>\s*<td>\s*<a rel="nofollow" href="(.*?)" class='result-link'>(.*?)</a>\s*</td>\s*</tr>\s*<tr>\s*<td>&nbsp;&nbsp;&nbsp;</td>\s*<td class='result-snippet'>\s*(.*?)\s*</td>\s*</tr>"#).unwrap();
}

pub async fn search(query: &Query) -> Result<Vec<SearchResult>, Error> {
    let url = format!("https://lite.duckduckgo.com/lite?q={}", query.query);
    let body = http_get_text(&url).await?;

    let mut results = Vec::new();

    for capture in RE.captures_iter(&body) {
        let url = capture[1].to_string();
        let title = capture[2].to_string();
        let snippet = capture[3].to_string();

        results.push(SearchResult {
            url,
            title,
            snippet
        });
    }

    Ok(results)
}
