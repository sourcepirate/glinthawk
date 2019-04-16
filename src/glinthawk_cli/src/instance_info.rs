//! the module contains codes for discovering instance data
//! and all assosiated metrics for cloudwatch.
use reqwest::Error;

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
                let instance_id = reqwest::get("http://instance-data/latest/meta-data/instance-id")?.text()?;
                Ok(InstanceIP::Amazon(amazon_ip, instance_id))
            },
            Err(_) => {
                let ideal = reqwest::get("http://ifconfig.me/ip")?.text()?;
                Ok(InstanceIP::Internal(ideal, String::from("None")))
            }
        }
    }

    pub fn get_ip(&self) -> String {
        match self {
            &InstanceIP::Amazon(ref ip, _) => ip.clone(),
            &InstanceIP::Internal(ref ip, _) => ip.clone()
        }
    }

    pub fn get_id(&self) -> String {
        match self {
            &InstanceIP::Amazon(_, ref id) => id.clone(),
            &InstanceIP::Internal(_, ref id) => id.clone()
        }
    }
}
