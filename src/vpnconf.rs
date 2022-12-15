use uuid::Uuid;
use std::fmt;


pub struct VPN {
    pub name: String,
    pub command: String,
}

pub struct VPNConfig {
    pub name: String,
    pub username: String,
    pub password: String,
    pub id: String,
    pub address: String,
    pub vpn_type: String,

}


impl VPNConfig {
    pub fn new(name: String, vpn_type: String, username: String, password: String, address: String) -> Self {
        VPNConfig {
            name,
            username,
            password,
            id: Uuid::new_v4().to_string(),
            address,
            vpn_type,
        }
    }


    pub fn to_string(&self) -> String {
        format!("vpn:\n  name : {}\n  ID : {}\n  Type : {}\n  Address : {}\n  user_name : {}\n  user_name : {}", self.name, self.id, self.vpn_type, self.address, &self.username, &self.password)
    }
}


impl fmt::Display for VPNConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_string().as_ref())
    }
}


impl VPN {
    pub fn new(name: String, command: String) -> Self {
        VPN {
            name,
            command,
        }
    }


    pub fn to_string(&self) -> String {
        format!("vpn_type: {}\n  vpn_commad: {}\n", self.name, self.command)
    }
}


impl fmt::Display for VPN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_string().as_ref())
    }
}