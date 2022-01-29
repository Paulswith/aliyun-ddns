/*
create at `2019-09-09` by `itachy`
*/
#[macro_use]
extern crate log;
extern crate url;
extern crate reqwest;

pub mod model;
mod network;
mod config;
mod handler;

use handler::{obtain_records, update_records};
use std::{path::PathBuf, fs};
use model::argument_model::Argument;
use network::public_ip_fetcher::get_public_ip;
use crate::model::initial_model::ConfigModel;
use anyhow::{Context, Result};


pub fn entry(argument: &Argument) -> Result<()> {
    app_log4rs_init(argument.log4rs_path())?;
    let ref config_model = app_config_init(argument.config_path())?;

    info!("\n--------------------------Task start:--------------------------");
    // 1. fetch public ip
    let public_ip = get_public_ip(config_model.ip_providers())?;
    info!("Fetch public ip: {}", public_ip);

    // 2. aliyun interaction
    match obtain_records::obtain_domain_records(config_model) {
        Err(err) => {
            panic!("Unable obtain domain records: {}, program exit", err);
        }
        Ok(ref records) if records.is_empty() => {
            panic!("Obtained domain records is empty! program exit");
        }
        Ok(ref records) => {
            info!("Obtained {} records", records.len());
            update_records::update_domain_record(config_model,
                                                 &public_ip,
                                                 records);
        }
    }
    Ok(())
}


fn app_log4rs_init(log4rs_path: &Option<PathBuf>) -> Result<()> {
    let mut path = PathBuf::from(config::general::DEFAULT_LOG4RS_PATH);
    if let Some(log4rs_path) = log4rs_path {
        path = log4rs_path.to_path_buf();
    }
    log4rs::init_file(&path, Default::default())
        .with_context(|| format!("Initialize log config error with config-path: {}", path.to_string_lossy()))?;
    Ok(())
}


fn app_config_init(config_path: &Option<PathBuf>) -> Result<ConfigModel> {
    let mut path = PathBuf::from(config::general::DEFAULT_CONFIG_PATH);
    if let Some(config_path) = config_path {
        path = config_path.to_path_buf();
    }
    let common_config_content = fs::read_to_string(&path)
        .with_context(|| format!("Unable read config path with config-path: {}", path.to_string_lossy()))?;
    let config = serde_yaml::from_str(&common_config_content)
        .with_context(|| format!("Unable analysis config_path with config-path: {}", path.to_string_lossy()))?;
    Ok(config)
}
