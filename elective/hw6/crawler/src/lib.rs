#![forbid(unsafe_code)]

use linkify::{LinkFinder, LinkKind};
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver};
use tokio::sync::Mutex;
use tokio::time;

const MAX_NUM_OF_SITES: usize = 100;
const NUM_OF_RETRIES: usize = 10;

#[derive(Clone, Default)]
pub struct Config {
    pub concurrent_requests: Option<usize>, // Probably size of thread pool
}

pub struct Page {
    pub url: String,
    pub body: String,
}

pub struct Crawler {
    config: Config,
}

impl Crawler {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn run(&mut self, site: String) -> Receiver<Page> {
        let (tx, rx) = channel(MAX_NUM_OF_SITES);
        let orders: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::from([site])));
        let seen_pages: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

        for _ in 0..self.config.concurrent_requests.unwrap_or(2) {
            let tx_clone = tx.clone();
            let orders_clone = orders.clone();
            let seen_pages_clone = seen_pages.clone();
            tokio::spawn(async move {
                let mut number_of_retries = 0;
                loop {
                    let next_url_opt = {
                        let mut guard = orders_clone.lock().await;
                        guard.pop_front()
                    };
                    let next_url = match next_url_opt {
                        Some(url) => {
                            number_of_retries = 0;
                            url
                        }
                        None => {
                            time::sleep(Duration::from_millis(100)).await;
                            number_of_retries += 1;
                            if number_of_retries == NUM_OF_RETRIES {
                                break;
                            } else {
                                continue;
                            }
                        }
                    };

                    {
                        let mut seen_pages = seen_pages_clone.lock().await;
                        match seen_pages.contains(&next_url) {
                            true => continue,
                            false => seen_pages.insert(next_url.clone()),
                        };
                    };

                    let text = reqwest::get(&next_url).await.unwrap().text().await.unwrap();

                    let mut finder = LinkFinder::new();
                    finder.kinds(&[LinkKind::Url]);

                    let links: Vec<String> = finder
                        .links(&text)
                        .map(|l| l.as_str().to_string())
                        .collect();

                    let mut guard = orders_clone.lock().await;
                    guard.extend(links);

                    let _ = tx_clone
                        .send(Page {
                            url: next_url,
                            body: text,
                        })
                        .await;
                }
            });
        }

        rx
    }
}
