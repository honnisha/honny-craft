use bincode::Options;
use common::network_messages::{ClentMessages, ClientLogin, ServerMessages};
use renet::{ClientAuthentication, RenetClient, RenetConnectionConfig, DefaultChannel};
use std::time::Duration;
use std::{net::UdpSocket, time::SystemTime};

use crate::console::console_handler::Console;

pub const PROTOCOL_ID: u64 = 7;
pub const CHANNEL_ID: u8 = 0;

fn get_network_client(ip_port: String, login: ClientLogin) -> RenetClient {
    let server_addr = ip_port.clone().parse().unwrap();

    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    let connection_config = RenetConnectionConfig::default();

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let client_id = current_time.as_millis() as u64;

    let authentication = ClientAuthentication::Unsecure {
        protocol_id: PROTOCOL_ID,
        client_id,
        server_addr,
        user_data: Some(login.to_netcode_user_data()),
    };
    let client = RenetClient::new(current_time, socket, connection_config, authentication).unwrap();
    client
}

pub struct NetworkClient {
    client: RenetClient,
}

impl NetworkClient {
    pub fn init(ip_port: String, login: String) -> Self {
        Console::send_message(format!("Start network client for {}", ip_port));
        NetworkClient {
            client: get_network_client(ip_port, ClientLogin(login)),
        }
    }

    pub fn update(&mut self, delta: f64) {
        if let Err(e) = self.client.update(Duration::from_secs_f64(delta)) {
            panic!("Connection error: {}", e);
        }

        if self.client.is_connected() {
            while let Some(message) = self.client.receive_message(CHANNEL_ID) {
                let data: ServerMessages = match bincode::options().deserialize(&message) {
                    Ok(d) => d,
                    Err(e) => {
                        Console::send_message(format!("Can't read a message: {:?}", e));
                        continue;
                    }
                };
                match data {
                    ServerMessages::ConsoleOutput { text } => Console::send_message(text),
                }
            }
        }
        self.client.send_packets().unwrap();
    }

    pub fn disconnect(&mut self) {
        self.client.disconnect();
        Console::send_message("Disconnected from the server".to_string());
    }

    pub fn send_console_command(&mut self, command: String) {
        match bincode::options().serialize(&ClentMessages::ConsoleCommand { command }) {
            Ok(message) => self.client.send_message(DefaultChannel::Reliable, message),
            Err(e) => {
                Console::send_message(format!("Error console command send: {:?}", e));
            }
        }
    }
}
