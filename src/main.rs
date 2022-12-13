mod vpnconf;
mod database;

use std::{env, fs};
use std::env::{current_dir, current_exe};
use std::f64::consts::E;
use std::fmt::Debug;
use std::path::Path;
use rusqlite::{params, Connection, Result};
use crate::Cmd::{Connect, Delete, Edit, Insert, Help, List, Version};

enum Cmd {
    Insert,
    Edit,
    Delete,
    Connect,
    Help,
    List,
    Version,
}

fn add_config() {}

fn connect() {}

fn edit() {}


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
        "-c" => Connect,
        "-e" => Edit,
        "-d" => Delete,
        "-l" => List,
        "-h" | "--help" => Help,
        "-v" | "--v" | "--version" => Version,

        _ => Connect,
    };



    match com {
        Insert if args.len() > 1 => add_config(),
        Connect if args.len() > 1 => connect(),
        Edit if args.len() > 1 => edit(),
        _ => println!("wrong input size or argument !!! ")
    }
}
