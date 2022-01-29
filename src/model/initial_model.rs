/*
create at `2019-09-09` by `itachy`
*/
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use anyhow::{anyhow, Result};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Root")]
pub struct ConfigModel {
    change_sub_domains: Vec<String>,
    ip_provider: Vec<IpProvider>,
    authorization: Authorization,
}

impl ConfigModel {
    pub fn ip_providers(&self) -> &Vec<IpProvider> {
        &self.ip_provider
    }

    pub fn change_sub_domains(&self) -> BTreeSet<String> {
        let mut sub_domains = BTreeSet::new();
        self.change_sub_domains.iter().for_each(|sub_domain| {
            sub_domains.insert(sub_domain.to_string());
        });
        sub_domains
    }
}

/* domain relations */
impl ConfigModel {
    pub fn access_key_id(&self) -> &str { &self.authorization.access_key_id }

    pub fn access_key_secret(&self) -> &str { &self.authorization.access_key_secret }

    pub fn region_id(&self) -> &str {
        &self.authorization.region_id
    }

    pub fn current_root_domain(&self) -> &str {
        &self.authorization.current_root_domain
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpProvider {
    provider: String,
    regex_pattern: String,
}

impl IpProvider {
    pub fn new(provider: String, regex_pattern: String) -> Self {
        Self {
            provider,
            regex_pattern
        }
    }

    pub fn provider(&self) -> &str { &self.provider }

    pub fn regex_pattern(&self) -> &str { &self.regex_pattern }

    pub fn crawl_then_regex(&self, client: &reqwest::Client) -> Result<String> {
        let mut rsp = client.get(self.provider()).send()?;
        let page_content = rsp.text()?;
        // pattern matching
        let pattern = self.regex_pattern();
        let regex_pattern = regex::Regex::new(pattern)?;
        match regex_pattern.captures(&page_content) {
            None => Err(anyhow!("Could not match ip with content: {}", page_content)),
            Some(res) => {
                match String::from(&res[1]) {
                    regex_str if !regex_str.is_empty() => Ok(regex_str),
                    _ => Err(anyhow!("Regex got a empty string"))
                }
            },
        }
    }
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Authorization {
    access_key_id: String,
    access_key_secret: String,
    current_root_domain: String,
    region_id: String,
}