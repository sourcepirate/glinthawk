//! the module contains codes for discovering instance data
//! and all assosiated metrics for cloudwatch.
use reqwest::Error;
use rusoto_core::credential::ChainProvider;
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_ec2::{DescribeTagsRequest, Filter};
use rusoto_ec2::{Ec2, Ec2Client};

#[derive(Debug, Clone)]
pub enum InstanceIP {
    // Amazon instance with ip, instance-id
    Amazon(String, String),
    // Any normal install instance with ip
    Internal(String, String),
}

impl InstanceIP {
    pub fn get() -> Result<Self, Error> {
        let result = reqwest::get("http://instance-data.ec2.internal");
        info!("Identifing ip address...");
        match result {
            Ok(_) => {
                let amazon_ip =
                    reqwest::get("http://instance-data/latest/meta-data/local-ipv4")?.text()?;
                let instance_id =
                    reqwest::get("http://instance-data/latest/meta-data/instance-id")?.text()?;
                Ok(InstanceIP::Amazon(amazon_ip, instance_id))
            }
            Err(_) => {
                let ideal = reqwest::get("http://ifconfig.me/ip")?.text()?;
                Ok(InstanceIP::Internal(ideal, String::from("None")))
            }
        }
    }

    pub fn get_ip(&self) -> String {
        match self {
            &InstanceIP::Amazon(ref ip, _) => ip.clone(),
            &InstanceIP::Internal(ref ip, _) => ip.clone(),
        }
    }

    pub fn get_id(&self) -> String {
        match self {
            &InstanceIP::Amazon(_, ref id) => id.clone(),
            &InstanceIP::Internal(_, ref id) => id.clone(),
        }
    }

    pub fn get_asg(&self) -> String {
        match self {
            &InstanceIP::Amazon(_, ref id) => {
                // get asg details from instance id
                let cred_provider = ChainProvider::new();
                let http_provider = HttpClient::new().expect("Failed creating new ec2 client");
                let client = Ec2Client::new_with(http_provider, cred_provider, Region::UsEast1);
                let tag_filter = Filter {
                    name: Some(String::from("resource-id")),
                    values: Some(vec![id.clone()]),
                };
                let tag_request = DescribeTagsRequest {
                    next_token: None,
                    max_results: None,
                    dry_run: None,
                    filters: Some(vec![tag_filter]),
                };

                let tag_result = client.describe_tags(tag_request);
                match tag_result.sync() {
                    Ok(result) => {
                        let descriptions = result.tags;
                        if let Some(lst) = descriptions {
                            if lst.len() > 0 {
                                for value in lst {
                                    if value.key.unwrap()
                                        == String::from("aws:autoscaling:groupName")
                                    {
                                        return value.value.unwrap_or(String::from("NotAttached"));
                                    }
                                }
                                return String::from("NotAttached");
                            } else {
                                return String::from("NotAttached");
                            }
                        } else {
                            return String::from("NotAttached");
                        }
                    }
                    Err(_) => String::from("NotAttached"),
                }
            }
            &InstanceIP::Internal(_, _) => String::from("NotAttached"),
        }
    }
}
