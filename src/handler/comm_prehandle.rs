use chrono::{DateTime, SecondsFormat, Utc};
use hmac::{Hmac, Mac};
use rand::Rng;
use sha1::Sha1;
use url::form_urlencoded::byte_serialize;

type HmacSha1 = Hmac<Sha1>;

pub(super) fn encrypt_by_hmac_sha1(with_key: &str, raw_content: &str) -> anyhow::Result<String> {
    let mut inst = HmacSha1::new_from_slice(with_key.as_bytes())?;
    inst.update(raw_content.as_bytes());
    let res_output = inst.finalize();
    Ok(base64::encode(res_output.into_bytes()))
}

pub(super) fn generate_utc_timestamp() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.to_rfc3339_opts(SecondsFormat::Secs, true).to_string()
}

pub(super) fn signature_nonce() -> String {
    let mut t = rand::thread_rng();
    // format!("{}", t.gen_range(100_000_000, 1_000_000_000 - 1))
    format!("{}", t.gen_range(100_000_000..=1_000_000_000 - 1))
}

// encode rule: https://help.aliyun.com/document_detail/29747.html
pub(super) fn percent_encode(raw_str: &str) -> String {
    let percent_str: String = byte_serialize(raw_str.as_bytes()).collect();
    percent_str
        .replace("+", "%20")
        .replace("*", "%2A")
        .replace("%7E", "~")
}

// TODO: add unit test
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_hmac_sha1() {
//         let key = "abcdefg";
//         let raw = "1234567890";
//         let r1 = encrypt_by_hmac_sha1(key, raw).unwrap();
//     }
// }
