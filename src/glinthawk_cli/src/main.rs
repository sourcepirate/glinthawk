extern crate chrono;
extern crate proc_watcher;
extern crate reqwest;
extern crate rusoto_cloudwatch;
extern crate rusoto_core;
extern crate rusoto_ec2;

#[macro_use]
extern crate log;

pub mod cloudwatch;
pub mod instance_info;

use instance_info::InstanceIP;
use proc_watcher::runner::run_watchers;
use rusoto_cloudwatch::CloudWatchClient;
use rusoto_core::credential::ChainProvider;
use rusoto_core::request::HttpClient;
use rusoto_core::Region;

pub const NAMESPACE: &'static str = "Glinthawk";

fn main() {
    let cred_provider = ChainProvider::new();
    let http_provider = HttpClient::new().expect("Failed new client");
    let client = CloudWatchClient::new_with(http_provider, cred_provider, Region::UsEast1);
    let (rx, _handle) = run_watchers();
    println!("Discovering ip address");

    let ip = match InstanceIP::get() {
        Ok(_ipaddr) => _ipaddr,
        Err(_) => {
            println!("Unable to discover instance!!");
            return;
        }
    };
    let asg: String = ip.get_asg();
    println!("Resolving topology: {}", asg);

    println!("Ip address: {:?}", ip);
    for metric in rx {
        println!("Metric: {:?}", metric);
        cloudwatch::put(asg.clone(), &client, ip.clone(), metric);
    }
}
