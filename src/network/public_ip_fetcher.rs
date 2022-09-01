use crate::model::initial_model::{ConfigModel, EthernetInfo, IpProvider, RunType};
use crate::network::basic;
use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use std::process::Command;

pub async fn get_public_ip(config: &ConfigModel) -> Result<String> {
    match config.run_type {
        RunType::EthernetInfo => match config.ethernet_info {
            None => bail!("Set run_type to ethernet_info but not provide correct ethernet config"),
            Some(ref eth) => fetch_pub_ip_by_cmd(eth).await,
        },
        RunType::IpProvider => match config.ip_provider {
            None => bail!("Set run_type to ip_provider but not config correct ip_providers"),
            Some(ref ip_providers) if ip_providers.len() == 0 => {
                bail!("Set run_type to ip_provider but ip_providers were empty")
            }
            Some(ref ip_providers) => fetch_pub_ip_by_ip_provider(ip_providers).await,
        },
    }
}

async fn fetch_pub_ip_by_ip_provider(ip_providers: &Vec<IpProvider>) -> Result<String> {
    assert!(ip_providers.len() > 0);
    let client = basic::construct_client(Some("https://www.baidu.com"))?;
    let mut error_vec = vec![];
    for ip_provider in ip_providers {
        debug!(
            "Use ip provider: {}, with regex_pattern: {}",
            ip_provider.provider, ip_provider.regex_pattern
        );
        match ip_provider.crawl_then_regex(&client).await {
            Ok(pub_ip) => return Ok(pub_ip),
            Err(err) => error_vec.push(format!(
                "Miss match from provider:{} cause {}",
                ip_provider.provider, err
            )),
        }
    }
    let error_str = error_vec.join("\n\n");
    Err(anyhow!(
        "Try {} providers but didn't match any public ip, details: {}",
        ip_providers.len(),
        error_str
    ))
}

async fn fetch_pub_ip_by_cmd(ethernet_info: &EthernetInfo) -> Result<String> {
    let ip_addr_res = Command::new("ip").arg("addr").output()?;
    log::debug!("ip_addr: \n{:#?}", ip_addr_res);

    let ip_addr_res = String::from_utf8_lossy(&ip_addr_res.stdout);
    let ref cap_ether_pattern = format!(r"(?s)(\d:\s{}:\s.*)", ethernet_info.ethernet);
    let re = Regex::new(&cap_ether_pattern)?;
    let caps = re.captures(&ip_addr_res).with_context(|| {
        format!(
            "May not found ether in ip_addr with pattern={}",
            cap_ether_pattern
        )
    })?;
    log::debug!("ether pattern={}, result={}", cap_ether_pattern, &caps[1]);

    let re = Regex::new(r"inet\s(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})")?;
    let caps = re
        .captures(&caps[1])
        .with_context(|| "Can not find ip in ethernet result")?;
    Ok(caps[1].to_string())
}
