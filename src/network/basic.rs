/*
create at `2019-09-09` by `itachy`
*/

use crate::config::const_config::*;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header::{REFERER, USER_AGENT};
use std::time::Duration;


/** desc: 构造一个客户端
 @Param:
 @return:
*/
pub fn construct_client(referer: Option<&str>) -> reqwest::Result<reqwest::Client> {
    let headers = get_request_header(referer);
    let timeout = Duration::from_secs(REQ_DEFAULT_TIME_OUT_SECOND);
    reqwest::ClientBuilder::new()
        .timeout(timeout)
        .default_headers(headers)
        .build()
}

/* -------------------------------------------private------------------------------------------- */
/** desc: 提供一个referer 返回header
 @Param:
 @return:
*/
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
