mod vpnconf;
mod database;

use std::{env, fs};
use std::env::{current_dir, current_exe};
use std::path::Path;
use rusqlite::{params, Connection, Result};


fn main() {


    let mut exe_dir = current_exe().unwrap();
    exe_dir.pop();
    let data_dir = format!("{}/DATA",exe_dir.display());

    let db = database::DB::open(data_dir,"vpn_db.db3".to_string()).unwrap();

    let vpn = vpnconf::VPNConfig::new("20vpn".to_string(), "Cisco".to_string(), "hosien".to_string(), "1234".to_string());


    println!("{}", vpn);


    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {}

    for i in 1..args.len() {
        println!("{}", args[i]);
    };
}
