#[macro_use]
extern crate log;
extern crate reqwest;
extern crate url;

mod config;
mod handler;
pub mod model;
mod network;

use crate::model::initial_model::ConfigModel;
use anyhow::{Context, Result};
use handler::{obtain_records, update_records};
use model::argument_model::Argument;
use network::public_ip_fetcher::get_public_ip;
use std::path::PathBuf;

pub async fn entry(argument: &Argument) -> Result<()> {
    app_log4rs_init(&argument.log4rs_path)?;
    let ref config_model = ConfigModel::from_file(&argument.config_path)?;
    info!("--------------------------Task start:--------------------------");
    // 1. fetch public ip
    let public_ip = get_public_ip(&config_model).await?;
    info!("Fetch public ip: {}", public_ip);
    if argument.dry_run {
        return Ok(());
    }
    // 2. aliyun interaction
    match obtain_records::obtain_domain_records(config_model).await {
        Err(err) => {
            panic!("Unable obtain domain records: {}, program exit", err);
        }
        Ok(ref records) if records.is_empty() => {
            panic!("Obtained domain records is empty! program exit");
        }
        Ok(ref records) => {
            info!("Obtained {} records", records.len());
            update_records::update_domain_record(config_model, &public_ip, records).await?;
        }
    }
    Ok(())
}

fn app_log4rs_init(log4rs_path: &Option<PathBuf>) -> Result<()> {
    let mut path = PathBuf::from(config::general::DEFAULT_LOG4RS_PATH);
    if let Some(log4rs_path) = log4rs_path {
        path = log4rs_path.to_path_buf();
    }
    log4rs::init_file(&path, Default::default()).with_context(|| {
        format!(
            "Initialize log config error with config-path: {}",
            path.to_string_lossy()
        )
    })?;
    Ok(())
}
