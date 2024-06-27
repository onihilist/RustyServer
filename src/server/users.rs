
#[derive(Debug)]
pub struct userData<'a> {
    pub userId: i32,
    pub hwid: &'a str,
    pub username: &'a str,
    pub pass: &'a str
}