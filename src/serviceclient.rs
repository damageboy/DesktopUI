use std::time::{SystemTime, Duration};
use std::collections::LinkedList;
use serde_json::{Map, Value};
use std::cell::Cell;

const SERVICE_TIMEOUT_MS: u64 = 1500;

/// Client that queries and caches JSON state from the service.
/// Currently it doesn't do much parsing since the JSON is just shoved
/// through to the JavaScript UI for most of what's done with it.
pub struct ServiceClient {
    refresh_base_urls: Vec<&'static str>,
    auth_token: String,
    port: u16,
    base_url: String,
    last_successful_refresh_time: SystemTime,
    state: Map<String, Value>,
    post_queue: LinkedList<(String, String)>,
}

#[cfg(target_os = "macos")]
pub fn get_auth_token_and_port() -> Option<(String, u16)> {
    let port = std::fs::read_to_string("/Library/Application Support/ZeroTier/One/zerotier-one.port").map_or(9993_u16, |port| u16::from_str_radix(port.as_str(), 10).unwrap_or(9993_u16) );

    let home = std::env::var("HOME");
    if home.is_ok() {
        let mut p = home.unwrap();
        p.push_str("/Library/Application Support/ZeroTier/One/authtoken.secret");
        let token = std::fs::read_to_string(p);
        if token.is_ok() {
            return Some((token.unwrap().trim().into(), port));
        }
    }

    let token = std::fs::read_to_string("/Library/Application Support/ZeroTier/One/authtoken.secret");
    if token.is_ok() {
        return Some((token.unwrap().trim().into(), port));
    }

    None
}

impl ServiceClient {
    pub fn new(refresh_base_urls: Vec<&'static str>) -> ServiceClient {
        ServiceClient{
            refresh_base_urls,
            auth_token: String::new(),
            port: 0,
            base_url: String::new(),
            last_successful_refresh_time: SystemTime::UNIX_EPOCH,
            state: Map::new(),
            post_queue: LinkedList::new(),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.port != 0 && !self.auth_token.is_empty()
    }

    pub fn is_online(&self) -> bool {
        self.is_initialized() && SystemTime::now().duration_since(self.last_successful_refresh_time).map_or(false, |d| d < Duration::from_millis(SERVICE_TIMEOUT_MS))
    }

    pub fn with<R, F: FnOnce(&Value) -> R>(&self, path: &str, f: F) -> R {
        let mut m = Some(&self.state);
        let v = Cell::new(&Value::Null);
        path.split('/').for_each(|s| {
            m.map_or_else(|| {
                v.replace(&Value::Null); // null if looking past tree of maps
            }, |mv| {
                mv.get(s).map_or_else(|| {
                    v.replace(&Value::Null); // null if key not found
                }, |nv| {
                    m = nv.as_object(); // sets to None if not a map
                    if m.is_none() {
                        v.replace(nv); // if not a map, it's a value
                    }
                });
            });
        });
        f(v.into_inner())
    }

    pub fn get(&self, path: &str) -> Value {
        self.with(path, |v| v.clone())
    }

    pub fn get_str(&self, path: &str) -> String {
        self.get(path).as_str().map_or_else(|| String::new(), |s| s.into())
    }

    pub fn networks(&self) -> Vec<(String, Map<String, Value>)> {
        let mut nw: Vec<(String, Map<String, Value>)> = Vec::new();
        self.with("/network", |nws| {
            let _ = nws.as_array().map(|a| a.iter().for_each(|network| {
                let _ = network.as_object().map(|network| {
                    network.get("id").map(|id| {
                        id.as_str().map(|id| {
                            nw.push((id.into(), network.clone()))
                        });
                    });
                });
            }));
        });
        nw.sort_by(|a, b| (*a).0.cmp(&((*b).0)) );
        nw
    }

    fn http_get(&self, path: &str) -> (u16, String) {
        if self.auth_token.is_empty() || self.base_url.is_empty() {
            (0, String::new())
        } else {
            ureq::get(format!("{}{}", self.base_url, path).as_str()).timeout(Duration::from_millis(SERVICE_TIMEOUT_MS)).set("X-ZT1-Auth", self.auth_token.as_str()).call().map_or_else(|_| {
                (0, String::new())
            }, |res| {
                let status = res.status();
                let body = res.into_string();
                body.map_or_else(|_| {
                    (0, String::new())
                }, |res| {
                    (status, res)
                })
            })
        }
    }

    fn http_post(&self, path: &str, payload: &str) -> (u16, String) {
        if self.auth_token.is_empty() || self.base_url.is_empty() {
            (0, String::new())
        } else {
            ureq::post(format!("{}{}", self.base_url, path).as_str()).timeout(Duration::from_millis(SERVICE_TIMEOUT_MS)).set("X-ZT1-Auth", self.auth_token.as_str()).send_string(payload).map_or_else(|_| {
                (0, String::new())
            }, |res| {
                let status = res.status();
                let body = res.into_string();
                body.map_or_else(|_| {
                    (0, String::new())
                }, |res| {
                    (status, res)
                })
            })
        }
    }

    pub fn enqueue_post(&mut self, path: String, payload: String) {
        self.post_queue.push_back((path, payload));
    }

    /// Check auth token and port for running service and update if changed.
    pub fn sync_client_config(&mut self) {
        get_auth_token_and_port().map(|token_port| {
            if self.auth_token != token_port.0 || self.port != token_port.1 {
                self.auth_token = token_port.0.clone();
                self.port = token_port.1;
                self.base_url = format!("http://127.0.0.1:{}/", self.port);
            }
        });
    }

    /// Send enqueued posts, if there are any.
    pub fn do_posts(&mut self) {
        if !self.is_initialized() {
            self.sync_client_config();
        }
        if self.is_initialized() {
            loop {
                let pq = self.post_queue.pop_front();
                if pq.is_some() {
                    let pq = pq.unwrap();
                    if self.http_post(pq.0.as_str(), pq.1.as_str()).0 != 200 {
                        self.post_queue.push_front(pq);
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    /// Submit queued posts and get current service state.
    pub fn sync(&mut self) {
        if !self.is_initialized() {
            self.sync_client_config();
        }
        if self.is_initialized() {
            let mut ok = true;
            for endpoint in self.refresh_base_urls.iter() {
                let data = self.http_get(*endpoint);
                if data.0 == 200 {
                    self.state.insert((*endpoint).into(), serde_json::from_str::<Value>(data.1.as_str()).unwrap_or(Value::Null));
                } else {
                    ok = false;
                }
            }
            if ok {
                self.last_successful_refresh_time = SystemTime::now();
            }
        }
    }
}
