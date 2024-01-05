use crate::search::{Query, SearchResult};

use reqwest::{Client, RequestBuilder};
use regex::Regex;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r#"</span></span></div></div></div></a></div><h2><a href="(.*?)" h="(.*?)">(.*?)</a></h2><div class="b_caption" role="contentinfo"><p class="b_lineclamp. b_algoSlug"><span class="algoSlug_icon" data-priority="2">(.*?)</span>(.*?)</p></div></li>"#).unwrap();
}

pub fn request(request: Client, query: &Query) -> RequestBuilder {
    request
        .get(format!("https://bing.com/search?q={}", query.query))
}

pub fn search_results(response: &String) -> Vec<SearchResult> {
    let mut results = Vec::new();

    for capture in RE.captures_iter(&response) {
        let url = capture[1].to_string();
        let title = capture[3].to_string();
        let snippet = capture[5].to_string();

        results.push(SearchResult {
            url,
            title,
            snippet
        });
    }

    results
}
