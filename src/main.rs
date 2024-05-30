mod utils;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use crate::utils::Logs::UtilsData;

fn handler(stream: TcpStream) {
    let user: SocketAddr = stream.local_addr().unwrap();
    let msgUser: String = format!("{} has connected successfully", user);
    let logs: UtilsData = utils::Logs::initLog(None, msgUser, None);
    utils::Logs::info(logs);
}

fn main() -> std::io::Result<()> {
    let logs: UtilsData = utils::Logs::initLog(None, "Server is starting...".to_string(), None);
    utils::Logs::info(logs);
    let addr: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let port: u16 = 42000;
    let socket_addr = SocketAddr::from(SocketAddr::new(addr, port));
    let listener: TcpListener = TcpListener::bind(socket_addr)?;
    let logs: UtilsData = utils::Logs::initLog(None, "Server is successfully started !".to_string(), None);
    utils::Logs::success(logs);
    let logs: UtilsData = utils::Logs::initLog(None, "Listening for a connection...".to_string(), None);
    utils::Logs::info(logs);

    for stream in listener.incoming() {
        handler(stream?);
    }
    Ok(())
}
