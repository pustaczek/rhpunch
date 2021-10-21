use rhpunch::{get_config_path, Request};
use rhpunch::{Response, ServerConfig};
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

#[derive(Debug)]
struct ActiveRequest {
    address: SocketAddr,
}

fn main() {
    let config_path = get_config_path();
    let config_bytes = std::fs::read(config_path).unwrap();
    let config: ServerConfig = toml::from_slice(&config_bytes).unwrap();
    println!("{:?}", config);
    let socket = UdpSocket::bind((config.listen_address, config.listen_port)).unwrap();
    let mut buf = [0; 2000];
    let mut active_requests: HashMap<String, ActiveRequest> = HashMap::new();
    loop {
        let (len, address) = socket.recv_from(&mut buf).unwrap();
        let request: Request = serde_json::from_slice(&buf[..len]).unwrap();
        println!("{:?} {:?}", request, address);
        if let Some(occupied) = active_requests.remove(&request.id) {
            println!("found entry {:?}", occupied);
            let first_response = Response { address };
            let second_response = Response {
                address: occupied.address,
            };
            socket
                .send_to(
                    &serde_json::to_vec(&first_response).unwrap(),
                    occupied.address,
                )
                .unwrap();
            socket
                .send_to(&serde_json::to_vec(&second_response).unwrap(), address)
                .unwrap();
        } else {
            println!("no entry, found, inserting...");
            active_requests.insert(request.peer_id, ActiveRequest { address });
        }
    }
}
