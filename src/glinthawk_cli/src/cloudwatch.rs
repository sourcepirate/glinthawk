use super::NAMESPACE;
use chrono::{SecondsFormat, Utc};
use proc_watcher::Metric;
use rusoto_cloudwatch::CloudWatch;
use rusoto_cloudwatch::CloudWatchClient;
use rusoto_cloudwatch::Dimension;
use rusoto_cloudwatch::MetricDatum;
use rusoto_cloudwatch::PutMetricDataInput;
use rusoto_core::credential::ChainProvider;
use rusoto_core::request::{HttpClient, HttpConfig};
use rusoto_core::Region;
use std::error::Error;

// Below function takes the ip address and metric information
// To injest the data into the cloudwatch by converting it into
// a standard metric format.
pub fn get_metric_info(ip: String, m: Metric) -> MetricDatum {
    let dim = Dimension {
        name: "IpAddress".to_owned(),
        value: ip,
    };
    let current_time = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    let dim = vec![dim];

    let (_, val) = m.to_pair();

    MetricDatum {
        value: Some(val),
        unit: Some(m.get_metric_unit()),
        timestamp: Some(current_time),
        metric_name: m.get_metric_name(),
        dimensions: Some(dim),
        counts: None,
        statistic_values: None,
        storage_resolution: None,
        values: None,
    }
}

// push mertic data
pub fn put_metric_data(ip: String, m: Metric) -> PutMetricDataInput {
    let info = get_metric_info(ip.clone(), m);
    PutMetricDataInput {
        namespace: NAMESPACE.to_owned(),
        metric_data: vec![info],
    }
}

// push client

pub fn put(ip: String, m: Metric) -> () {
    let cred_provider = ChainProvider::new();
    let http_provider = HttpClient::new().expect("Failed new client");
    let client = CloudWatchClient::new_with(http_provider, cred_provider, Region::UsEast1);
    let data = put_metric_data(ip, m);
    let result = client.put_metric_data(data);
    match result.sync() {
        Ok(_) => println!("Published!!"),
        Err(_e) => println!("{}", _e),
    };
}
