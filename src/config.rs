pub mod const_config {
    pub const HEADER_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) \
    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.142 Safari/537.36";

    pub const REQ_DEFAULT_TIME_OUT_SECOND: u64 = 5;

    pub const ALIYUN_DNS_DOMAIN: &str = "https://alidns.aliyuncs.com";
}

pub mod general {
    /* default log4rs path */
    pub const DEFAULT_LOG4RS_PATH: &str = "common_conf/log4rs.yaml";

    /* default config path */
    pub const DEFAULT_CONFIG_PATH: &str = "common_conf/config.yaml";
}

pub mod param {
    pub const HTTP_METHOD: &str         = "GET";

    // addition pagesize:
    pub const K_PAGE_SIZE: &str         = "PageSize";
    pub const V_PAGE_SIZE: &str         = "100";

    // cause large pagesize, fetch first page number is enough
    pub const K_PAGE_NUMBER: &str       = "PageNumber";
    pub const V_PAGE_NUMBER: &str       = "1";

    // default json:
    pub const K_FORMAT: &str            = "Format";
    pub const V_FORMAT: &str            = "JSON";

    pub const K_VERSION: &str           = "Version";
    pub const V_VERSION: &str           = "2015-01-09";

    pub const K_ACTION: &str            = "Action";

    pub const K_ACCESS_KEY_ID: &str     = "AccessKeyId";

    pub const K_DOMAIN_NAME: &str       = "DomainName";

    pub const K_REGION_ID: &str         = "RegionId";

    pub const K_RECORD_ID: &str         = "RecordId";

    pub const K_SIGNATURE: &str         = "Signature";

    pub const K_SIGNATURE_METHOD: &str  = "SignatureMethod";
    pub const V_SIGNATURE_METHOD: &str  = "HMAC-SHA1";

    pub const K_TIMESTAMP: &str         = "Timestamp";

    pub const K_VALUE: &str             = "Value";

    pub const K_RR: &str                = "RR";

    pub const K_TYPE: &str              = "Type";

    pub const K_SIGNATURE_VERSION: &str = "SignatureVersion";
    pub const V_SIGNATURE_VERSION: &str = "1.0";

    pub const K_SIGNATURE_NONCE: &str   = "SignatureNonce";

}

pub mod handle_action {
    // request for obtain all domain records
    pub const AC_DESCRIBE_DOMAIN_RECORDS: &str = "DescribeDomainRecords";
    // request for update single domain record
    pub const AC_UPDATE_DOMAIN_RECORD: &str = "UpdateDomainRecord";
}
