use reqwest::Error;

pub enum InstanceIP {
    Amazon(String),
    Internal(String),
}

impl InstanceIP {
    pub fn get() -> Result<Self, Error> {
        let result = reqwest::get("http://instance-data.ec2.internal");
        info!("Identifing ip address...");
        match result {
            Ok(_) => {
                let amazon =
                    reqwest::get("http://instance-data/latest/meta-data/local-ipv4")?.text()?;
                Ok(InstanceIP::Amazon(amazon))
            }
            Err(_) => {
                let ideal = reqwest::get("http://ifconfig.me")?.text()?;
                Ok(InstanceIP::Internal(ideal))
            }
        }
    }
}
