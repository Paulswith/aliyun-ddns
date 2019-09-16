/*
create at `2019-09-09` by `itachy`
*/
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
extern crate url;
extern crate reqwest;

pub mod model;
mod derived;
mod network;
mod config;
mod handler;

use handler::{obtain_records, update_records};
use std::{path::PathBuf, process::exit, fs};
use model::argument_model::Argument;
use crate::model::initial_model::ConfigModel;


pub fn entry(argument: &Argument) {
    app_log4rs_init(argument.log4rs_path());
    let ref config_model = app_config_init(argument.config_path());
    info!("\n--------------------------Task start:--------------------------");
    // 1. fetch public ip
    let public_ip = network::public_ip_fetcher::get_public_ip(
        config_model.ip_providers());
    if let None = public_ip {
        error!("Unable fetch public ip, program exit");
        exit(1);
    }
    let public_ip = public_ip.unwrap();
    info!("Fetch public ip: {}", public_ip);
    // 2. aliyun interaction
    match obtain_records::obtain_domain_records(config_model) {
        Err(err) => {
            error!("Unable obtain domain records: {}, program exit", err);
            exit(1);
        },
        Ok(ref records) if records.is_empty() => {
            info!("Obtained domain records is empty! program exit");
            exit(1);
        },
        Ok(ref records) => {
            info!("Obtained {} records", records.len());
            update_records::update_domain_record(config_model,
                                                 &public_ip,
                                                 records);
        }
    }
}

fn app_log4rs_init(log4rs_path: &Option<PathBuf>) {
    let mut path = PathBuf::from(config::general::DEFAULT_LOG4RS_PATH);
    if let Some(log4rs_path) = log4rs_path {
        path = log4rs_path.to_path_buf();
    }
    log4rs::init_file(path, Default::default())
    .expect("Initialize log config error");
}

fn app_config_init(config_path: &Option<PathBuf>) -> ConfigModel {
    let mut path = PathBuf::from(config::general::DEFAULT_CONFIG_PATH);
    if let Some(config_path) = config_path {
        path = config_path.to_path_buf();
    }
    let common_config_content= fs::read_to_string(path)
        .expect("Unable read config path");
    serde_yaml::from_str(&common_config_content)
        .expect("Unable analysis config_path")
}
