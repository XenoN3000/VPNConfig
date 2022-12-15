use std::borrow::{Borrow, BorrowMut};
use std::fs;
use std::path::Path;
use rusqlite::{params, Connection, Result, Error, CachedStatement, Map};
use crate::vpnconf::{VPN, VPNConfig};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use uuid::Uuid;
use crate::database::Opration::{Delete, Insert, Select};
use crate::database::Table::{VPNConfigT, VpnT};
use crate::database::Transaction::Action;


#[derive(Hash, Eq, PartialEq)]
enum Opration {
    Insert = 0,
    Update,
    Delete,
    Select,
    None,
}

#[derive(Hash, Eq, PartialEq)]
enum Table {
    VpnT,
    VPNConfigT,
    None,

}


#[derive(Hash, Eq, PartialEq)]
enum Transaction {
    Action(Table, Opration),
    None,
}


pub struct DB {
    connection: Connection,
    path: String,
    queries: HashMap<Transaction, String>,

}

impl DB {
    pub fn new(dir: String, db_file_name: String) -> Result<Self, Error> {
        if !Path::new(dir.as_str()).is_dir() {
            fs::create_dir(dir.as_str()).unwrap();
        }

        let path_to_db = format!("{}/{}", dir, db_file_name);
        let conn = Connection::open(path_to_db.to_string()).unwrap();

        let mut queries: HashMap<Transaction, String> = HashMap::from([
            (Action(VpnT, Insert), "INSERT INTO VPNs (
                        name,
                        command
                        ) VALUES (?, ?); ".to_string()),
            (Action(VPNConfigT, Insert), "INSERT INTO VPNConfig (
                        name ,
                        username,
                        password,
                        id,
                        vpn_type,
                        address
                        ) VALUES (?, ?, ?, ? ,?,?); ".to_string()),
            (Action(VpnT, Delete), "DELETE FROM VPNs WHERE name = ?".to_string()),
            (Action(VPNConfigT, Delete), "DELETE FROM VPNConfig WHERE name = ?".to_string()),
            (Action(VpnT, Select), "SELECT * FROM VPNs WHERE name = ?".to_string()),
            (Action(VPNConfigT, Select), "SELECT * FROM VPNConfig WHERE name = ?".to_string())
        ]);

        let db = DB {
            connection: conn,
            path: path_to_db.to_string(),
            queries,
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
        let mut table = self.connection.prepare_cached(self.queries[&Action(VpnT, Insert)].as_str())?;
        table.insert([vpn.name, vpn.command]).unwrap();
        return Ok(());
    }


    pub fn insert_vpn_config(&self, vpn_config: VPNConfig) -> Result<()> {
        let mut table = self.connection.prepare_cached(self.queries[&Action(VPNConfigT, Insert)].as_str())?;
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


    pub fn get_vpn_config(&self, name: String) -> Result<VPNConfig, Error> {
        let query = self.queries[&Action(VPNConfigT, Select)].as_str();
        let mut table = self.connection.prepare_cached(query)?;
        let mut rows = table.query([name])?;
        let row = rows.next().unwrap().unwrap();

        let conf = VPNConfig {
            name: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            id: row.get(3)?,
            vpn_type: row.get(4)?,
            address: row.get(5)?,

        };

        return Ok(conf);
    }


    pub fn get_vpn(&self, name: String) -> Result<VPN, Error> {
        let query = self.queries[&Action(VpnT, Select)].as_str();
        let mut table = self.connection.prepare_cached(query)?;
        let mut rows = table.query([name])?;
        let row = rows.next().unwrap().unwrap();

        let vpn = VPN {
            name: row.get(0)?,
            command: row.get(1)?,
        };
        return Ok(vpn);
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }
}



