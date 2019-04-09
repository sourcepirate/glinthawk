extern crate chrono;
extern crate proc_watcher;
extern crate reqwest;
extern crate rusoto_cloudwatch;
extern crate rusoto_core;
#[macro_use]
extern crate log;

pub mod cloudwatch;
pub mod instance_info;

use instance_info::InstanceIP;
use proc_watcher::runner::run_watchers;

pub const NAMESPACE: &'static str = "Glinthawk";

fn main() {
    let (rx, handle) = run_watchers();
    println!("Discovering ip address");
    let ip = match InstanceIP::get() {
        Ok(_ipaddr) => match _ipaddr {
            InstanceIP::Amazon(r) => r.clone(),
            InstanceIP::Internal(i) => i.clone(),
        },
        Err(_) => {
            println!("Unable to discover instance!!");
            return;
        }
    };
    println!("Ip address: {}", ip);
    for metric in rx {
        println!("Metric: {:?}", metric);
        let measurement = cloudwatch::put(ip.clone(), metric);
    }
}
