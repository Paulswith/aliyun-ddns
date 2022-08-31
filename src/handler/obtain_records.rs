use super::construct_request::generate_request_uri;
use crate::config::handle_action::AC_DESCRIBE_DOMAIN_RECORDS;
use crate::model::{
    initial_model::ConfigModel, record_model::DomainRecordModel, record_model::Record,
};
use crate::network::basic::construct_client;
use anyhow::{anyhow, Result};

pub async fn obtain_domain_records(config_model: &ConfigModel) -> Result<Vec<Record>> {
    let root_domain = config_model.current_root_domain();
    let request_url =
        generate_request_uri(config_model, root_domain, AC_DESCRIBE_DOMAIN_RECORDS, None);
    debug!("Obtain domain records with url: {}", request_url);
    //TODO: fetch more page
    let client = construct_client(None)?;
    let res = client.get(&request_url).send().await?;
    match res.status() {
        reqwest::StatusCode::OK => {
            let model: DomainRecordModel = res.json().await?;
            Ok(model.records())
        }
        status_code => Err(anyhow!(
            "Request domain records got wrong status-code: {}",
            status_code
        )),
    }
}
