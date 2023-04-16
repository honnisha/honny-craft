use std::{
    net::UdpSocket,
    time::{Duration, Instant, SystemTime}, thread,
};

use clap::Parser;
use renet::{
    generate_random_bytes, RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig, ServerEvent,
};

const PROTOCOL_ID: u64 = 7;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct MainCommand {
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    ip: String,

    #[arg(short, long, default_value_t = String::from("14191"))]
    port: String,
}

fn main() {
    let args = MainCommand::parse();

    println!("HonnyCraft Server version {}", VERSION);

    let ip_port = format!("{}:{}", args.ip, args.port);

    //let socket = UdpSocket::bind(ip_port.clone()).unwrap();
    //let server_addr = socket.local_addr().unwrap();
    let server_addr = ip_port.parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();

    let delta_time = Duration::from_millis(16);
    let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let mut server = RenetServer::new(current_time, server_config, connection_config, socket).unwrap();
    let channel_id = 0;

    println!("Server started in {}", ip_port);

    let mut last_updated = Instant::now();

    // Your gameplay loop
    loop {
        // Receive new messages and update clients
        let now = Instant::now();
        server.update(now - last_updated).unwrap();
        last_updated = now;

        // Check for client connections/disconnections
        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected(id, user_data) => {
                    println!("Client {} connected", id);
                }
                ServerEvent::ClientDisconnected(id) => {
                    println!("Client {} disconnected", id);
                }
            }
        }

        // Receive message from channel
        for client_id in server.clients_id().into_iter() {
            while let Some(message) = server.receive_message(client_id, channel_id) {
                // Handle received message
            }
        }

        // Send a text message for all clients
        server.broadcast_message(channel_id, "server message".as_bytes().to_vec());

        // Send message to only one client
        //let client_id = ...;
        //server.send_message(client_id, channel_id, "server message".as_bytes().to_vec());

        // Send packets to clients
        server.send_packets().unwrap();
        thread::sleep(Duration::from_millis(50));
    }
}