mod utils;
mod server;

use std::io::{Read, Write};
use std::convert::TryFrom;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use crate::utils::Logs::UtilsData;

fn handler(mut stream: TcpStream) -> std::io::Result<()> {

    let mut buffer = [0; 512];
    let user: SocketAddr = stream.local_addr().unwrap();
    let msgUser: String = format!("{} has connected successfully", user);
    let logs: UtilsData = utils::Logs::initLog(None, msgUser, None);
    utils::Logs::info(logs);

    loop {

        let bytes_read = stream.read(&mut buffer)?;

        // Convert the buffer to a string slice
        match std::str::from_utf8(&buffer[..bytes_read]) {
            Ok(msg) => {
                if msg == "ABC" {
                    if let Err(e) = write_to_client(&mut stream, "OK".as_bytes()) {
                        let logs = utils::Logs::initLog(None, format!("Failed to write to server: {}", e), None);
                        utils::Logs::error(logs);
                    }
                } else {
                    if let Err(e) = write_to_client(&mut stream, "Unknown packet".as_bytes()) {
                        let logs = utils::Logs::initLog(None, format!("Failed to write to server: {}", e), None);
                        utils::Logs::error(logs);
                    }
                }
            }
            Err(e) => {
                let logs = utils::Logs::initLog(None, format!("Invalid UTF-8 sequence: {}", e), None);
                utils::Logs::error(logs);
            }
        }
    }
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

fn write_to_client(stream: &mut impl Write, data: &[u8]) -> io::Result<()> {
    stream.write_all(data)
}