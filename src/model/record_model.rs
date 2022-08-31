use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Root")]
pub struct DomainRecordModel {
    #[serde(rename = "PageNumber")]
    page_number: i64,
    #[serde(rename = "TotalCount")]
    total_count: i64,
    #[serde(rename = "PageSize")]
    page_size: i64,
    #[serde(rename = "RequestId")]
    request_id: String,
    #[serde(rename = "DomainRecords")]
    domain_records: DomainRecords,
}

impl DomainRecordModel {
    pub fn records(&self) -> Vec<Record> {
        self.domain_records.record.to_vec()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DomainRecords {
    #[serde(rename = "Record")]
    record: Vec<Record>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    #[serde(rename = "RR")]
    rr: String,         // nec
    #[serde(rename = "Status")]
    status: Option<String>,
    #[serde(rename = "Value")]
    value: String,      // nec
    #[serde(rename = "Weight")]
    weight: Option<i64>,
    #[serde(rename = "RecordId")]
    record_id: String,  // nec
    #[serde(rename = "Type")]
    type_field: String, // nec
    #[serde(rename = "DomainName")]
    domain_name: Option<String>,
    #[serde(rename = "Locked")]
    locked: Option<bool>,
    #[serde(rename = "Line")]
    line: Option<String>,
    #[serde(rename = "TTL")]
    ttl: Option<i64>,
    #[serde(rename = "Priority")]
    priority: Option<i64>,
}

impl Record {
    pub fn rr(&self) -> &str { &self.rr }

    pub fn type_field(&self) -> &str { &self.type_field }

    pub fn record_id(&self) -> &str { &self.record_id }

    // Compare self.value(ip format) with other ip
    pub fn is_value_equal(&self, value: &str) -> bool { self.value == value }
}
