/*
create at `2019-09-11` by `itachy`
*/

use crate::model::initial_model::ConfigModel;
use crate::config::{param::*, const_config::*};
use super::comm_prehandle::*;
use std::collections::BTreeMap;


pub fn generate_request_uri(config_model: &ConfigModel,
                            domain_name: &str,
                            action: &str,
                            custom_params: Option<BTreeMap<&'static str, String>>) -> String {
    // append '&' to secret:
    let ref access_key_secret = format!("{}&", config_model.access_key_secret());
    let mut param_tree = generate_param(config_model,
                                        domain_name,
                                        action,
                                        custom_params);
    // signature
    supply_signature(&mut param_tree, access_key_secret);
    info!("Complete params: {:?}", param_tree);
    let complete_url_query_params = flat_to_url_query_param(&param_tree);
    format!("{}/?{}", ALIYUN_DNS_DOMAIN, complete_url_query_params)
}

/** desc: generate url query param without 'signature'
 use BTree cause generate 'signature' need sorted param-key
*/
fn generate_param(config_model: &ConfigModel,
                  domain_name: &str,
                  action: &str,
                  custom_params: Option<BTreeMap<&'static str, String>>)
    -> BTreeMap<&'static str, String>{
    let access_key_id = config_model.access_key_id().to_string();
    let region_id = config_model.region_id().to_string();
    let signature_nonce = signature_nonce();
    let timestamp = generate_utc_timestamp();
    // BTree
    let mut params = BTreeMap::new();

    // supply custom kv
    if let Some(custom_params) = custom_params {
        for (k, v) in custom_params {
            params.insert(k, v);
        }
    }

    params.insert(K_ACTION, action.to_string());            // not public param
    params.insert(K_DOMAIN_NAME, domain_name.to_string());  // domain name
    // generate params
    params.insert(K_REGION_ID, region_id);
    params.insert(K_TIMESTAMP, timestamp);
    params.insert(K_ACCESS_KEY_ID, access_key_id);
    params.insert(K_SIGNATURE_NONCE, signature_nonce);
    // default params
    params.insert(K_FORMAT, V_FORMAT.to_string());
    params.insert(K_PAGE_NUMBER, V_PAGE_NUMBER.to_string());
    params.insert(K_PAGE_SIZE, V_PAGE_SIZE.to_string());
    params.insert(K_VERSION, V_VERSION.to_string());
    params.insert(K_SIGNATURE_METHOD, V_SIGNATURE_METHOD.to_string());
    params.insert(K_SIGNATURE_VERSION, V_SIGNATURE_VERSION.to_string());
    params
}

/** desc: flat btree to url query param(with url-encode(aliyun encode rule))
*/
fn flat_to_url_query_param(params: &BTreeMap<&str, String>) -> String {
    let flat_param_vec: Vec<_> = params.into_iter()
        .map(|(k, v)|
            format!("{}={}", percent_encode(k), percent_encode(v)))
        .collect();
    flat_param_vec.join("&")
}

/** desc: supply the signature param(follow aliyun encrypt rule)
*/
fn supply_signature(params: &mut BTreeMap<&str, String>, signature_key_secret: &str) {
    // 1. generate canonicalized query string(url-encode each key and value) btree auto sorted by key
    let canonicalized_query_string = flat_to_url_query_param(params);
    debug!("Canonicalized_query_string: {}", canonicalized_query_string);
    // 2. generate string to sign(total 'canonicalized_query_string' url-encode again)
    let string_to_sign = format!("{}&{}&{}", percent_encode(HTTP_METHOD),
                                 percent_encode("/"),
                                 percent_encode(&canonicalized_query_string));
    debug!("String_to_sign: {}", string_to_sign);
    // 3. signature: (Hmac<sha1> + base64encode)
    let signature = encrypt_by_hmac_sha1(signature_key_secret, &string_to_sign);
    debug!("Signature: {}", signature);
    params.insert(K_SIGNATURE, signature);
}