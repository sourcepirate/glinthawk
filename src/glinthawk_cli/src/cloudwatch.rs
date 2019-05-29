use super::NAMESPACE;
use crate::instance_info::InstanceIP;
use proc_watcher::Metric;
use rusoto_cloudwatch::CloudWatch;
use rusoto_cloudwatch::CloudWatchClient;
use rusoto_cloudwatch::Dimension;
use rusoto_cloudwatch::MetricDatum;
use rusoto_cloudwatch::PutMetricDataInput;

// Below function takes the ip address and metric information
// To injest the data into the cloudwatch by converting it into
// a standard metric format.
pub fn get_instance_metric_info(asg: String, ip: InstanceIP, m: Vec<Metric>) -> Vec<MetricDatum> {
    let dim1 = Dimension {
        name: "IpAddress".to_owned(),
        value: ip.get_ip(),
    };
    let dim2 = Dimension {
        name: "InstanceId".to_owned(),
        value: ip.get_id(),
    };
    let dim3 = Dimension {
        name: "Asg".to_owned(),
        value: asg,
    };
    // let current_time = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    let dim = vec![dim1, dim2, dim3];

    m.iter()
        .map(|x| {
            let (_, val) = x.to_pair();

            MetricDatum {
                value: Some(val),
                unit: Some(x.get_metric_unit()),
                timestamp: Some(x.get_time_stamp()),
                metric_name: x.get_metric_name(),
                dimensions: Some(dim.clone()),
                counts: None,
                statistic_values: None,
                storage_resolution: None,
                values: None,
            }
        })
        .collect()
}

// get asg metric data
pub fn get_asg_metric_info(asg: String, m: Vec<Metric>) -> Vec<MetricDatum> {
    let dim = Dimension {
        name: "AutoScalingGroup".to_owned(),
        value: asg,
    };
    // let current_time = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    let dimvec = vec![dim];
    m.into_iter()
        .map(|x| {
            let (_, val) = x.to_pair();
            MetricDatum {
                value: Some(val),
                unit: Some(x.get_metric_unit()),
                timestamp: Some(x.get_time_stamp()),
                metric_name: x.get_metric_name(),
                dimensions: Some(dimvec.clone()),
                counts: None,
                statistic_values: None,
                storage_resolution: None,
                values: None,
            }
        })
        .collect()
}

// push mertic data
pub fn put_instance_metric_data(asg: String, ip: InstanceIP, m: Vec<Metric>) -> PutMetricDataInput {
    let info: Vec<MetricDatum> = get_instance_metric_info(asg, ip, m);
    PutMetricDataInput {
        namespace: NAMESPACE.to_owned(),
        metric_data: info,
    }
}

pub fn put_asg_metric_data(asg: String, m: Vec<Metric>) -> PutMetricDataInput {
    let info: Vec<MetricDatum> = get_asg_metric_info(asg, m);
    PutMetricDataInput {
        namespace: NAMESPACE.to_owned(),
        metric_data: info,
    }
}

// push client

pub fn put(asg: String, client: &CloudWatchClient, instance: InstanceIP, m: Vec<Metric>) -> () {
    let data = put_instance_metric_data(asg.clone(), instance, m.clone());
    let datasg = put_asg_metric_data(asg.clone(), m.clone());
    let result = client.put_metric_data(data);
    match result
        .sync()
        .and_then(|_e| client.put_metric_data(datasg).sync())
    {
        Ok(_) => println!("Published!!"),
        Err(_e) => println!("{}", _e),
    };
}
