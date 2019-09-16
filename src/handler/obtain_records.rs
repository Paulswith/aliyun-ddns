/*
create at `2019-09-11` by `itachy`
*/
use super::construct_request::generate_request_url;
use crate::model::{initial_model::ConfigModel,
                   record_model::DomainRecordModel,
                   record_model::Record};
use crate::config::handle_action::AC_DESCRIBE_DOMAIN_RECORDS;
use crate::network::basic::construct_client;
use crate::derived::errors;


pub fn obtain_domain_records(config_model: &ConfigModel) -> errors::Result<Vec<Record>> {
    let root_domain = config_model.current_root_domain();
    let request_url = generate_request_url(config_model,
                                           root_domain,
                                           AC_DESCRIBE_DOMAIN_RECORDS,
                                           None);
    debug!("Obtain domain records with url: {}", request_url);
    //TODO: fetch more page
    let client= construct_client(None)?;
    let mut res = client.get(&request_url).send()?;
    if res.status() != reqwest::StatusCode::OK {
        bail!(errors::ErrorKind::ResponseWithErrorStatus(res.status().to_string()));
    }
    let model: DomainRecordModel = res.json()?;
    Ok(model.records())
}
