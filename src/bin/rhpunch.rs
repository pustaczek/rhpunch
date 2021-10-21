use rhpunch::{get_config_path, Request};
use rhpunch::{ClientConfig, Response};
use std::net::{SocketAddr, UdpSocket};

fn main() {
    let config_path = get_config_path();
    let config_bytes = std::fs::read(config_path).unwrap();
    let config: ClientConfig = toml::from_slice(&config_bytes).unwrap();
    println!("{:?}", config);
    let server_addr = SocketAddr::new(config.server_address, config.server_port);
    let server_socket = UdpSocket::bind(("0.0.0.0", 0)).unwrap();
    let request = Request {
        id: config.id,
        peer_id: config.peer_id,
    };
    server_socket
        .send_to(&serde_json::to_vec(&request).unwrap(), &server_addr)
        .unwrap();
    let mut buf = [0; 2000];
    let len = server_socket.recv(&mut buf).unwrap();
    let response: Response = serde_json::from_slice(&buf[..len]).unwrap();
    println!("{:?}", response);
}
