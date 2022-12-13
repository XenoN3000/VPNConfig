use uuid::Uuid;
use std::fmt;

struct VPNConfig {
    pub name: String,
    pub username: String,
    pub password: String,
    pub id: Uuid,
    vpn_type: String,
}



impl VPNConfig {
    pub fn new(name: String, vpn_type: String, username: String, password: String) -> Self {
        VPNConfig {
            name,
            username,
            password,
            id: Uuid::new_v4(),
            vpn_type,
        }
    }


    pub fn to_string(&self) -> String {
        format!("vpn:\n  name : {}\n  ID : {}\n  Type : {}\n  user_name : {}\n  user_name : {}", self.name, self.id, self.vpn_type, &self.username, &self.password)
    }
}




impl fmt::Display for VPNConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_string().as_ref())
    }
}
