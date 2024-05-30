use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};

fn handler(stream: TcpStream) {
    println!("Someone connected.");
}

fn main() -> std::io::Result<()> {
    let addr: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let port: u16 = 42000;
    let socket_addr = SocketAddr::from(SocketAddr::new(addr, port));
    let listener: TcpListener = TcpListener::bind(socket_addr)?;
    println!("Listening for a connection...");

    for stream in listener.incoming() {
        handler(stream?);
    }
    Ok(())
}
