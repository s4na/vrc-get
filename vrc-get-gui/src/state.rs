mod changes;
mod config;
mod packages;
mod settings;
mod templates;
mod unity;
mod updater;

pub use changes::*;
pub use config::*;
pub use packages::*;
pub use settings::*;
pub use templates::*;
pub use unity::*;
pub use updater::*;

struct AnyHost;

impl<'a> PartialEq<&'a str> for AnyHost {
    fn eq(&self, _other: &&'a str) -> bool {
        true
    }
}

fn retry_policy() -> reqwest::retry::Builder {
    reqwest::retry::for_host(AnyHost)
        .max_retries_per_request(2)
        .classify_fn(|req_rep| {
            if req_rep.method() == reqwest::Method::GET && req_rep.error().is_some() {
                req_rep.retryable()
            } else {
                req_rep.success()
            }
        })
}

pub fn new_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .retry(retry_policy())
        .user_agent(concat!(
            "vrc-get-gui/",
            env!("CARGO_PKG_VERSION"),
            " (A GUI version of vrc-get, A.K.K, ALCOM) (",
            env!("CARGO_PKG_HOMEPAGE"),
            ")"
        ))
        // https://github.com/vrc-get/vrc-get/issues/2653
        // IDK why but it might take over 10 sec to connect / read
        .connect_timeout(std::time::Duration::from_secs(60))
        .read_timeout(std::time::Duration::from_secs(60))
        .timeout(std::time::Duration::from_secs(10 * 60)) // 10 minutes
        .build()
        .expect("building client")
}
