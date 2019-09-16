/*
create at `2019-09-09` by `itachy`
*/
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;


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
    pub fn provider(&self) -> &str { &self.provider }

    pub fn regex_pattern(&self) -> &str { &self.regex_pattern }
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Authorization {
    access_key_id: String,
    access_key_secret: String,
    current_root_domain: String,
    region_id: String,
}