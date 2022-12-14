use std::fs;
use std::path::Path;
use rusqlite::{params, Connection, Result, Error};
use crate::vpnconf::{VPN, VPNConfig};


pub struct DB {
    pub connection: Connection,
    path: String,
}

impl DB {
    pub fn new(dir: String, db_file_name: String) -> Result<Self, Error> {
        if !Path::new(dir.as_str()).is_dir() {
            fs::create_dir(dir.as_str()).unwrap();
        }
        let path_to_db = format!("{}/{}", dir, db_file_name);
        let conn = Connection::open(path_to_db.to_string()).unwrap();
        let db = DB {
            connection: conn,
            path: path_to_db.to_string(),
        };
        DB::create_tables(&db);

        return Ok(db);
    }

    fn create_tables(db: &DB) {
        db.connection.execute(
            "CREATE TABLE IF NOT EXISTS VPNConfig (
                    name text uniqe not null,
                    username text,
                    password text,
                    id text PRIMARY KEY,
                    vpn_type text,
                    address text,
                    FOREIGN KEY (vpn_type) REFERENCES VPNs(name)
            );",
            (),
        ).unwrap();


        db.connection.execute(
            "CREATE TABLE IF NOT EXISTS VPNs (
                    name text PRIMARY KEY,
                    command text
            );
            ",
            (),
        ).unwrap();
    }

    pub fn open(dir: String, db_file_name: String) -> Result<Self, Error> {
        DB::new(dir, db_file_name)
    }


    pub fn insert_vpn(&self, vpn: VPN) -> Result<()> {
        let mut table = self.connection.prepare_cached(
            "INSERT INTO VPNs (
                        name,
                        command
                        ) VALUES (?, ?); ")?;
        table.insert([vpn.name,vpn.command]).unwrap();
        return Ok(());
    }


    pub fn insert_vpn_config(&self, vpn_config: VPNConfig) -> Result<()> {
        let mut table = self.connection.prepare_cached(
            "INSERT INTO VPNConfig (
                        name ,
                        username,
                        password,
                        id,
                        vpn_type,
                        address
                        ) VALUES (?, ?, ?, ? ,?,?); ")?;
        table.insert([
            vpn_config.name,
            vpn_config.username,
            vpn_config.password,
            vpn_config.id.to_string(),
            vpn_config.vpn_type,
            vpn_config.address
        ]).unwrap();
        return Ok(());
    }


    pub fn path(&self) -> String {
        self.path.clone()
    }
}

