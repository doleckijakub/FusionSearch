pub mod engines;

// Actix query wrapper

use actix_web::web::Query as ActixQuery;
use std::collections::HashMap;
pub type ActixQueryWrapper = ActixQuery<HashMap<String, String>>;

// Query

use log::info;

pub struct Query {
    pub query: String,
}

impl From<&mut ActixQueryWrapper> for Query {
    fn from(request: &mut ActixQueryWrapper) -> Query {
        let query = request.remove("q").unwrap();

        if !request.is_empty() {
            info!("Unparsed request parameters: {:?}", request.keys());
        }

        Query {
            query
        }
    }
}

// SearchResult
    
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub snippet: String,
}

// Error

pub enum Error {
    // UrlParseError,
    // TooManyRequestsInWindow,
    SendError,
    ReadError,
}

// sliding windows

/* struct HostAccessCount {
    count: u32,
    expiration: Instant,
}

struct SlidingWindow {
    window: HashMap<String, HostAccessCount>,
}

impl SlidingWindow {
    fn new() -> SlidingWindow {
        SlidingWindow {
            window: HashMap::new(),
        }
    }

    fn increment_count(&mut self, host: String) -> bool {
        let now = Instant::now();
        let entry = self.window.entry(host.clone()).or_insert(HostAccessCount {
            count: 0,
            expiration: now + WINDOW_DURATION,
        });

        if entry.expiration < now {
            entry.count = 0;
            entry.expiration = now + WINDOW_DURATION;
        }

        entry.count += 1;
        entry.count > MAX_REQUESTS_PER_WINDOW
    }
}

lazy_static::lazy_static! {
    static ref SLIDING_WINDOWS: Mutex<HashMap<String, SlidingWindow>> = Mutex::new(HashMap::new());
}

const WINDOW_DURATION: Duration = Duration::from_secs(20);
const MAX_REQUESTS_PER_WINDOW: u32 = 15;
const CACHE_DURATION: Duration = Duration::from_secs(3600);

use std::collections::VecDeque;
use std::hash::Hash;

struct Cache<K: Eq + Hash + Clone, V> {
    queue: VecDeque<(K, Instant, V)>,
    map: HashMap<K, usize>,
}

impl<K: Eq + Hash + Clone, V> Cache<K, V> {
    fn new() -> Self {
        Self { queue: VecDeque::new(), map: HashMap::new() }
    }

    fn update(&mut self) {
        let now = Instant::now();
        
        while self.queue.front().is_some() && self.queue.front().unwrap().1 > now {
            let (key, _, _) = self.queue.pop_front().unwrap();
            self.map.remove(&key);
        }
    }

    fn put(&mut self, key: K, value: V) {
        self.update();
        self.queue.push_back((key.clone(), Instant::now() + CACHE_DURATION, value));
        self.map.insert(key, self.queue.len() - 1);
    }

    fn get(&self, key: K) -> Option<&V> {
        if let Some(n) = self.map.get(&key) {
            if let Some((_, _, v)) = self.queue.get(*n) {
                return Some(v);
            }
        }
        None
    }
}

lazy_static::lazy_static! {
    static ref CACHE: Mutex<Cache<String, String>> = Mutex::new(Cache::new());
} */

// user agents

use rand::prelude::SliceRandom;
lazy_static::lazy_static! {
    static ref BROWSER_USER_AGENTS: [reqwest::header::HeaderValue; 5] = [
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.9999.99 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:99.0) Gecko/20100101 Firefox/99.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/99.0.9999.99 Safari/605.1.15",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.9999.99 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0",
    ].map(|user_agent| user_agent.parse::<_>().unwrap());
}

// http get

use reqwest::RequestBuilder;

pub async fn http_get_text(request: RequestBuilder) -> Result<String, Error> {
    // TODO: reimplement caching
    /* let url = Url::parse(&url).map_err(|_err| Error::UrlParseError)?;

    if let Some(cached) = CACHE.lock().unwrap().get(url.as_str().to_string()) {
        println!("Accessing \"{url}\" from cache...");
        return Ok(cached.to_string())
    }

    let too_many_requests = {
        let host: String = url.host_str().unwrap().into();

        let mut sliding_windows_lock = SLIDING_WINDOWS.lock().unwrap();
        let sliding_window: &mut SlidingWindow = sliding_windows_lock
            .entry(host.clone())
            .or_insert_with(SlidingWindow::new);

        sliding_window.increment_count(host.clone())
    };

    if too_many_requests {
        return Err(Error::TooManyRequestsInWindow);
    } */

    let user_agent = BROWSER_USER_AGENTS
        .choose(&mut rand::thread_rng()).unwrap();
    let response = request
        .header(reqwest::header::USER_AGENT, user_agent)
        .send()
        .await
        .map_err(|_err| Error::SendError)?;
    let text = response
        .text()
        .await
        .map_err(|_err| Error::ReadError)?;

    // CACHE.lock().unwrap().put(url.as_str().to_string(), text.clone());

    Ok(text)
}
