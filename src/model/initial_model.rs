use crate::config;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, fs, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Root")]
pub struct ConfigModel {
    pub(crate) change_sub_domains: Vec<String>,
    pub(crate) authorization: Authorization,

    pub(crate) run_type: RunType,
    pub(crate) ip_provider: Option<Vec<IpProvider>>,
    pub(crate) ethernet_info: Option<EthernetInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
    pub(crate) access_key_id: String,
    pub(crate) access_key_secret: String,
    pub(crate) current_root_domain: String,
    pub(crate) region_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum RunType {
    #[serde(rename = "ip_provider")]
    IpProvider,
    #[serde(rename = "ethernet_info")]
    EthernetInfo,
}

impl ConfigModel {
    pub fn from_file(config_path: &Option<PathBuf>) -> Result<ConfigModel> {
        let mut path = PathBuf::from(config::general::DEFAULT_CONFIG_PATH);
        if let Some(config_path) = config_path {
            path = config_path.to_path_buf();
        }
        let yaml_ct = fs::read_to_string(&path).with_context(|| {
            format!(
                "Unable read config path with config-path: {}",
                path.to_string_lossy()
            )
        })?;
        let config = Self::from_str(&yaml_ct)
            .with_context(|| format!("parse as ConfigModel fail with content: \n{}", yaml_ct))?;
        Ok(config)
    }

    pub fn from_str(content: &str) -> Result<ConfigModel> {
        let config = serde_yaml::from_str(&content)?;
        Ok(config)
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
    pub fn access_key_id(&self) -> &str {
        &self.authorization.access_key_id
    }

    pub fn access_key_secret(&self) -> &str {
        &self.authorization.access_key_secret
    }

    pub fn region_id(&self) -> &str {
        &self.authorization.region_id
    }

    pub fn current_root_domain(&self) -> &str {
        &self.authorization.current_root_domain
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpProvider {
    pub(crate) provider: String,
    pub(crate) regex_pattern: String,
}

impl IpProvider {
    pub async fn crawl_then_regex(&self, client: &reqwest::Client) -> Result<String> {
        let rsp = client.get(&self.provider).send().await?;
        let page_content = rsp.text().await?;
        // pattern matching
        let regex_pattern = regex::Regex::new(&self.regex_pattern)?;
        match regex_pattern.captures(&page_content) {
            None => Err(anyhow!("Could not match ip with content: {}", page_content)),
            Some(res) => match String::from(&res[1]) {
                regex_str if !regex_str.is_empty() => Ok(regex_str),
                _ => Err(anyhow!("Regex got a empty string")),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EthernetInfo {
    pub(crate) ethernet: String,
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_initial_model_parser() {
        let yaml_ct = r#"
change_sub_domains:
  - 'hello' # hello.infot.me
  - 'world' # world.infot.me

authorization:
  access_key_id: 'IAMasd879679mSAD9'
  access_key_secret: 'IAMasd879679mSAD9IAMasd879679mSAD9'
  current_root_domain: 'infot.me'                   # the root-domain
  region_id: 'cn-shenzhen'                          # city code, China cities default is cn-<city-full-name>

run_type: ethernet_info
ip_provider:
  -
    provider: 'https://www.ip.cn'
    regex_pattern: '(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})'

ethernet_info:
  ethernet: 'ppp0'"#;
        let c1 = ConfigModel::from_str(yaml_ct).unwrap();
        let c2 = ConfigModel {
            change_sub_domains: vec!["hello".to_string(), "world".to_string()],
            authorization: Authorization {
                access_key_id: "IAMasd879679mSAD9".to_string(),
                access_key_secret: "IAMasd879679mSAD9IAMasd879679mSAD9".to_string(),
                current_root_domain: "infot.me".to_string(),
                region_id: "cn-shenzhen".to_string(),
            },
            run_type: RunType::EthernetInfo,
            ip_provider: Some(vec![IpProvider {
                provider: "https://www.ip.cn".to_string(),
                regex_pattern: r"(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})".to_string(),
            }]),
            ethernet_info: Some(EthernetInfo {
                ethernet: "ppp0".to_string(),
            }),
        };
        assert_eq!(c1, c2)
    }
}
