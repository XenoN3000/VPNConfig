mod vpnconf;
use std::env;
use std::ops::Add;
use std::str;
use rusqlite::{params, Connection, Result};


fn main() -> Result<()> {
    let connection = Connection::open_in_memory()?;


    connection.execute(
        "CREATE TABLE VPNConfig (
            name text uniqe not null,
            username text,
            password text,
            id text PRIMARY KEY
        ); ", (), )?;


    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {}

    for i in 1..args.len() {
        println!("{}", args[i]);
    };

    return Ok(());
}
