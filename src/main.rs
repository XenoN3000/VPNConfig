mod vpnconf;
mod database;

use std::env;
use std::env::{current_exe};
use crate::Cmd::{Connect, Delete, Edit, Insert, Help, List, Version, InsertVPN};
use crate::database::DB;
use crate::vpnconf::{VPNConfig, VPN};

enum Cmd {
    Insert,
    InsertVPN,
    Edit,
    Delete,
    Connect,
    Help,
    List,
    Version,
}

fn add_config(db: DB, name: String, username: String, password: String, vpn_type: String, address: String) {
    let vconf = VPNConfig::new(name, vpn_type, username, password, address);


    let result = db.insert_vpn_config(vconf);
}

fn add_vpn(db: DB, name: String, command: String) {
    let vpn = VPN::new(name,command);
    let result = db.insert_vpn(vpn);

}

fn connect(name: String) {}

fn edit() {}

fn delete(name: String) {}

fn version() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("vpn connector - version {}", VERSION);
}

fn list_vpn_and_configs() {}

fn help() {}


fn main() {
    let mut exe_dir = current_exe().unwrap();
    exe_dir.pop();
    let data_dir = format!("{}/DATA", exe_dir.display());

    let db = database::DB::open(data_dir, "vpn_db.db3".to_string()).unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("wrong input size !!!");
        return;
    }


    let com = match args[1].as_str() {
        "-a" => Insert,
        "-A" => InsertVPN,
        "-c" | "--connect" => Connect,
        "-e" => Edit,
        "-d" => Delete,
        "-l" => List,
        "-h" | "--help" => Help,
        "-v" | "--v" | "--version" => Version,

        _ => Connect,
    };


    match com {
        Insert  if args.len() == 7 => add_config(db, args[2].to_string(), args[3].to_string(), args[4].to_string(), args[5].to_string(), args[6].to_string()),
        InsertVPN if args.len() == 4 => add_vpn(db, args[2].to_string(),args[3].to_string()),
        Connect if args.len() == 3 => connect(args[2].to_string()),
        Connect if args.len() == 2 => connect(args[1].to_string()),
        Delete  if args.len() == 3 => delete(args[2].to_string()),
        Delete  if args.len() == 2 => delete(args[1].to_string()),
        List => list_vpn_and_configs(),
        Version => version(),

        Edit if args.len() > 1 => edit(),
        _ => println!("wrong input size or argument !!! ")
    }
}
