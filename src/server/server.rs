#![feature(io_error_more)]

use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use crate::{
    core::config::Config,
    server::structures::{handshake::Handshake, packet::Packet, status::StatusResponse},
};

use super::error::ServerError;

pub struct ProxyServer {
    listener: TcpListener,
}

impl ProxyServer {
    pub fn new(config: &Config) -> Result<Self, ServerError> {
        let listener = TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))?;
        //listener.set_nonblocking(true)?;

        Ok(Self { listener })
    }

    pub fn start(&self) -> Result<(), ServerError> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => match stream.peer_addr() {
                    Ok(addr) => {
                        println!("New connection: {}", addr);
                        thread::spawn(move || {
                            ProxyServer::handle_client(&stream).expect("failed to handle client")
                        });
                    }
                    Err(e) => {
                        println!("ERROR: failed to ask address for connection: {}", e);
                    }
                },
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::AddrInUse
                        || e.kind() == std::io::ErrorKind::AddrNotAvailable
                    {
                        panic!("Failed to start TCP server: {}", e);
                    }
                    println!("Error: {}", e);
                }
            }
        }
        Ok(())
    }

    fn handle_client(mut stream: &TcpStream) -> Result<(), ServerError> {
        let _: Packet<Handshake> = Packet::from_stream(stream)?;

        let status = StatusResponse::default_fake();
        let mut response_paquet = Packet::from(status);

        response_paquet.send(&mut stream)?;

        /*let response = serde_json::to_string(&status)?;
        let mut response_size: Vec<u8> = VarInt::from(response.len() as u32).into();

        let mut vec = Vec::new();
        vec.push(0u8); // paquet ID
        vec.append(&mut response_size); // response size
        vec.append(&mut response.as_bytes().to_vec()); // response

        let mut p_size: Vec<u8> = VarInt::from(vec.len() as u32).into(); // paquet size
        p_size.append(&mut vec); // merge paquet size + payload (id + data)

        println!("DATA: {:?}", p_size);

        stream.write_all(&mut p_size)?;*/

        /*let mut data = [0 as u8; 150];

        while match stream.read(&mut data) {
            Ok(size) => {
                // echo everything
                if size > 0 {
                    println!("Data: {:?}", &data[0..size]);
                }
                stream.write(&data[0..size])?;
                true
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {}*/

        Ok(())
    }
}
