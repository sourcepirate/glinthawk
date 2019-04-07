extern crate chrono;
extern crate proc_watcher;
extern crate reqwest;
extern crate rusoto_cloudwatch;
#[macro_use]
extern crate log;

pub mod cloudwatch;
pub mod instance_info;

pub const NAMESPACE: &'static str = "Glinthawk";

fn main() {
    println!("Glinthawk!!");
}
