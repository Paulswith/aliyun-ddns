/*
create at `2019-09-09` by `itachy`
*/

use crate::network::basic;
use crate::model::initial_model::IpProvider;
use crate::derived::errors;


pub fn get_public_ip(ip_providers: &Vec<IpProvider>) -> Option<String> {
    match basic::construct_client(None) {
        Err(err) => {
            error!("Unable construct client: {}", err);
            None
        },
        Ok(client) => {
            for ip_provider in ip_providers {
                debug!("Use ip provider: {}, with regex_pattern: {}",
                       ip_provider.provider(), ip_provider.regex_pattern());
                match crawl_from_ip_cn(&client, ip_provider) {
                    Ok(ref public_ip) if !public_ip.is_empty() => {
                        return Some(public_ip.to_string());
                    },
                    Err(err) => {
                        debug!("Unable found public with return error: {}", err);
                    },
                    _ =>  {
                        debug!("Found one empty public ip.");
                    }
                }
            }
            return None;
        }
    }
}

/* --------------------fetchers--------------------- */
fn crawl_from_ip_cn(client: &reqwest::Client, ip_provider: &IpProvider) -> errors::Result<String> {
    let mut rsp = client.get(ip_provider.provider()).send()?;
    let page_content = rsp.text()?;
    // pattern matching
    let pattern = ip_provider.regex_pattern();
    let regex_pattern = regex::Regex::new(pattern)?;
    match regex_pattern.captures(&page_content) {
        None => bail!(errors::ErrorKind::IPV4NotFoundError),
        Some(res) => Ok(String::from(&res[1])),
    }
}

