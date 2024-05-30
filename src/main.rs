mod utils;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use crate::utils::Logs::UtilsData;

fn handler(stream: TcpStream) {
    let mut user: SocketAddr = stream.local_addr().unwrap();
    let msgUser: &str = format!("{} has connected successfully", user).as_str();
    let mut logs: UtilsData = utils::Logs::initLog(None, msgUser, None);
    utils::Logs::info(logs);
}

fn main() -> std::io::Result<()> {
    let mut logs: UtilsData = utils::Logs::initLog(None, "Server is starting...", None);
    utils::Logs::info(logs);
    let addr: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let port: u16 = 42000;
    let socket_addr = SocketAddr::from(SocketAddr::new(addr, port));
    let listener: TcpListener = TcpListener::bind(socket_addr)?;
    let mut logs: UtilsData = utils::Logs::initLog(None, "Server is successfully started !", None);
    utils::Logs::success(logs);
    let mut logs: UtilsData = utils::Logs::initLog(None, "Listening for a connection...", None);
    utils::Logs::info(logs);

    for stream in listener.incoming() {
        handler(stream?);
    }
    Ok(())
}
