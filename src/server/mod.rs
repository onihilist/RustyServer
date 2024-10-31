
pub mod protocols;
pub mod response;
pub mod users;

use std::io;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use crate::{database, server, utils};
use crate::server::protocols::{checkProtocol, protocolData};
use crate::server::response::responseData;
use crate::utils::Logs::UtilsData;

fn handler(mut stream: TcpStream) -> std::io::Result<()> {

    let mut buffer = [0; 512];
    let user: SocketAddr = stream.local_addr().unwrap();
    let logs: UtilsData = utils::Logs::initLog(None, format!("{} has connected successfully", user), None);
    utils::Logs::info(logs);

    loop {

        let bytes_read = stream.read(&mut buffer)?;

        match std::str::from_utf8(&buffer[..bytes_read]) {
            Ok(msg) => {

                let protocolReceive: protocolData = server::protocols::createProtocol(msg.to_string());
                let response: responseData = server::response::initResponseData(checkProtocol(protocolReceive));

                write_to_client(&mut stream, response).expect("TODO: panic message");

            }
            Err(e) => {
                let logs = utils::Logs::initLog(None, format!("Invalid UTF-8 sequence: {}", e), None);
                utils::Logs::error(logs);
            }
        }
    }
}

pub async fn startServer() -> std::io::Result<()> {

    let logs: UtilsData = utils::Logs::initLog(None, "Server is starting...".to_string(), None);
    utils::Logs::info(logs);

    let addr: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let port: u16 = 42000;

    let logs: UtilsData = utils::Logs::initLog(None, "Trying to connected with database...".to_string(), None);
    utils::Logs::info(logs);

    database::database::connectToDB().await;

    let socket_addr = SocketAddr::from(SocketAddr::new(addr, port));
    let listener: TcpListener = TcpListener::bind(socket_addr)?;

    let logs: UtilsData = utils::Logs::initLog(None, "Server has successfully started !".to_string(), None);
    utils::Logs::success(logs);

    let logs: UtilsData = utils::Logs::initLog(None, "Listening for a connection...".to_string(), None);
    utils::Logs::info(logs);

    for stream in listener.incoming() {
        handler(stream?);
    }
    Ok(())
}

fn write_to_client(stream: &mut impl Write, data: responseData) -> io::Result<()> {
    let (protocol_bytes, sender_bytes, receiver_bytes, data_bytes) = data.to_byte_slices();
    let payload = server::protocols::concatenate_slices(
        b"::",
        protocol_bytes,
        sender_bytes,
        receiver_bytes,
        data_bytes
    );
    stream.write_all(payload)
}