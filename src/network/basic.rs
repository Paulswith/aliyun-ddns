/*
create at `2019-09-09` by `itachy`
*/

use crate::config::const_config::*;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header::{REFERER, USER_AGENT};
use std::time::Duration;


/// make a full http-client with default headers
///
/// # Arguments
///
/// * `referer`:
///
/// returns: Result<Client, Error>
pub fn construct_client(referer: Option<&str>) -> reqwest::Result<reqwest::Client> {
    let headers = get_request_header(referer);
    let timeout = Duration::from_secs(REQ_DEFAULT_TIME_OUT_SECOND);
    reqwest::ClientBuilder::new()
        .timeout(timeout)
        .default_headers(headers)
        .build()
}

/// make a http header map
/// # Arguments
///
/// * `referer`: http-header referer
///
/// returns: HeaderMap<HeaderValue>
fn get_request_header(referer: Option<&str>) -> HeaderMap {
    let mut header = HeaderMap::new();
    match HeaderValue::from_str(HEADER_USER_AGENT) {
        Err(_) => error!("Setting USER_AGENT failed"),
        Ok(user_agent) => { header.insert(USER_AGENT,user_agent); },
    }
    if referer.is_none() {
        warn!("Ignore referer setting cause its param is none.");
        header
    } else {
        match HeaderValue::from_str(referer.unwrap()) {
            Err(_) => error!("Unable setting REFERER"),
            Ok(referer) => { header.insert(REFERER, referer); },
        }
        header
    }
}
