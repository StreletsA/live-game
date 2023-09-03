use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::common::game::live_game::LiveGame;

pub struct LiveGameServer {
    game: LiveGame,
}

impl LiveGameServer {
    pub fn new() -> Self {
        let game = LiveGame::new(100, 100);

        return LiveGameServer { game };
    }

    pub fn start(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_client(stream)
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            }
        }
    }

    fn handle_read(&self, mut stream: &TcpStream) {
        let mut buf = [0u8; 4096];
        match stream.read(&mut buf) {
            Ok(_) => {
                let req_str = String::from_utf8_lossy(&buf);
                println!("{}", req_str);
            }
            Err(e) => println!("Unable to read stream: {}", e),
        }
    }

    fn handle_write(&self, mut stream: TcpStream) {
        let mut message: String = "HTTP/1.1 200 OK\r\nContent-Type: text/json; charset=UTF-8\r\n\r\n<html><body>".to_string();
        message.push_str(self.game.get_generation().to_string().as_str());
        message.push_str("</body></html>\r\n");

        let response = message.as_bytes();
        match stream.write(response) {
            Ok(_) => println!("Response sent"),
            Err(e) => println!("Failed sending response: {}", e),
        }
    }

    fn handle_client(&mut self, stream: TcpStream) {
        self.handle_read(&stream);
        self.game.next_gen();
        self.handle_write(stream);
    }
}