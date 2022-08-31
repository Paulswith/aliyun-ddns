use super::construct_request::generate_request_uri;
use crate::config::{handle_action::AC_UPDATE_DOMAIN_RECORD, param::*};
use crate::model::{initial_model::ConfigModel, record_model::Record};
use crate::network::basic::construct_client;
use std::collections::BTreeMap;

pub async fn update_domain_record(
    config_model: &ConfigModel,
    bind_pub_ip: &str,
    domain_records: &Vec<Record>,
) {
    let root_domain = config_model.current_root_domain();
    let change_sub_domains = config_model.change_sub_domains();
    let mut change_counter = 0;
    for record in domain_records {
        if change_sub_domains.contains(record.rr()) && !record.is_value_equal(bind_pub_ip) {
            info!("Match one record need update: {}", record.rr());
            // PRE-UPDATE
            let custom_param = construct_addition_param(bind_pub_ip, record);
            let request_url = generate_request_uri(
                config_model,
                root_domain,
                AC_UPDATE_DOMAIN_RECORD,
                Some(custom_param),
            );
            debug!("Obtain domain records with url: {}", request_url);
            // UPDATING
            match update_single_domain_record(&request_url).await {
                Err(err) => error!("Update single domain record encounter error: {}", err),
                Ok(status_code) if status_code != reqwest::StatusCode::OK => {
                    error!("Update single domain record encounter error from server")
                }
                _ => {
                    info!("Update single domain record successfully.");
                    change_counter += 1;
                }
            }
        }
    }
    info!(
        "Task done, successfully changed {} record(s).",
        change_counter
    );
}

fn construct_addition_param(bind_pub_ip: &str, record: &Record) -> BTreeMap<&'static str, String> {
    let mut custom_param = BTreeMap::new();
    custom_param.insert(K_VALUE, bind_pub_ip.to_string());
    custom_param.insert(K_TYPE, record.type_field().to_string());
    custom_param.insert(K_RR, record.rr().to_string());
    custom_param.insert(K_RECORD_ID, record.record_id().to_string());
    custom_param
}

async fn update_single_domain_record(url: &str) -> reqwest::Result<reqwest::StatusCode> {
    let client = construct_client(None)?;
    let res = client.get(url).send().await?;
    // let text: String = res.text()?;
    let status = res.status();
    debug!("Response status:{:?}", status);
    // debug!("Response status:{:?}, result: {}", status, text);
    Ok(status)
}
