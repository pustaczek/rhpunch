use serde::Deserialize;
use serde::Serialize;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct ClientConfig {
    pub listen_address: IpAddr,
    pub listen_port: u16,
    pub server_address: IpAddr,
    pub server_port: u16,
    pub id: String,
    pub peer_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub listen_address: IpAddr,
    pub listen_port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub id: String,
    pub peer_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub address: SocketAddr,
}

pub fn get_config_path() -> PathBuf {
    let mut args = std::env::args_os();
    if args.len() != 2 {
        eprintln!("\x1B[1;31merror:\x1B[0m config file path not specified as argument");
    }
    args.next().unwrap();
    PathBuf::from(args.next().unwrap())
}
