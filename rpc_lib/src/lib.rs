#![allow(unused_variables)]
use std::net::IpAddr;

use serde::{self, Deserialize, Serialize};

jsonrpsee::rpc_api! {
    pub Api {
        fn show_routes() -> Vec<LearnedRoute>;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LearnedRoute {
    pub source: String,
    pub afi: String,
    pub safi: String,
    pub received_at: i64,
    pub age: String,
    pub prefix: String,
    pub next_hop: Option<IpAddr>,
    pub origin: String,
    pub as_path: String,
    pub local_pref: Option<u32>,
    pub multi_exit_disc: Option<u32>,
    pub communities: Vec<String>,
}
