/*
create at `2019-09-09` by `itachy`
*/

use crate::network::basic;
use crate::model::initial_model::IpProvider;
use anyhow::{anyhow, Result};


pub fn get_public_ip(ip_providers: &Vec<IpProvider>) -> Result<String> {
    assert!(ip_providers.len() > 0);
    let client = basic::construct_client(Some("https://www.baidu.com"))?;
    let mut error_vec = vec![];
    for ip_provider in ip_providers {
        debug!("Use ip provider: {}, with regex_pattern: {}",
               ip_provider.provider(), ip_provider.regex_pattern());
        match ip_provider.crawl_then_regex(&client) {
            Ok(pub_ip) => return Ok(pub_ip),
            Err(err) =>
                error_vec.push(
                    format!("Miss match from provider:{} cause {}", ip_provider.provider(), err)
                ),
        }
    }
    let error_str = error_vec.join("\n\n");
    Err(anyhow!(
        "Try {} providers but didn't match any public ip, details: {}", ip_providers.len(), error_str)
    )
}
