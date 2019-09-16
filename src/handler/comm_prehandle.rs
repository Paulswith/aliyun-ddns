/*
create at `2019-09-11` by `itachy`
*/
//extern crate url;
use crypto::{hmac::Hmac, sha1::Sha1, mac::Mac};
use chrono::{DateTime, SecondsFormat, Utc};
use rand::Rng;
use url::form_urlencoded::byte_serialize;


pub(super) fn encrypt_by_hmac_sha1(with_key: &str, raw_content: &str) -> String {
    let sha1_encryptor = Sha1::new();
    let mut encryptor = Hmac::new(sha1_encryptor,
                                        with_key.as_bytes());
    encryptor.input(raw_content.as_bytes());
    let res = encryptor.result();
    base64::encode(res.code())
}

pub(super) fn generate_utc_timestamp() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.to_rfc3339_opts(SecondsFormat::Secs, true).to_string()
}

pub(super) fn signature_nonce() -> String {
    let mut t = rand::thread_rng();
    format!("{}", t.gen_range(100_000_000, 1_000_000_000 - 1))
}

// encode rule: https://help.aliyun.com/document_detail/29747.html
pub(super) fn percent_encode(raw_str: &str) -> String {
    let percent_str: String = byte_serialize(raw_str.as_bytes()).collect();
    percent_str.replace("+", "%20")
        .replace("*", "%2A")
        .replace("%7E", "~")
}