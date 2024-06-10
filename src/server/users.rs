
pub struct UserData {
    id: u32,
    pub_id: u32,
    username: &'static str,
    mac_addr: &'static str,
    hwid: &'static str,
    password: &'static str,
    creation_ts: u64
}