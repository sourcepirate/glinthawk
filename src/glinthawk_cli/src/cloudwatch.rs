use chrono::Utc;
use proc_watcher::Metric;
use rusoto_cloudwatch::Dimension;
use rusoto_cloudwatch::MetricDatum;

// Below function takes the ip address and metric information
// To injest the data into the cloudwatch by converting it into
// a standard metric format.
pub fn get_metric_info(ip: String, m: Metric) -> MetricDatum {
    let dim = Dimension {
        name: "IpAddress".to_owned(),
        value: ip,
    };
    let current_time = Utc::now().to_rfc3339();
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
