use std::io::Read;

use crate::utils::{string::DataString, varint::VarInt};

use super::MCPacket;

#[derive(Debug)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: DataString,
    pub server_port: u16,
    pub next_state: VarInt,
}

impl Default for Handshake {
    fn default() -> Self {
        Self {
            protocol_version: Default::default(),
            server_address: Default::default(),
            server_port: Default::default(),
            next_state: Default::default(),
        }
    }
}

impl MCPacket for Handshake {
    const ID: VarInt = VarInt::ZERO;

    fn from_stream(mut stream: &std::net::TcpStream) -> Result<Handshake, std::io::Error> {
        let mut h = Self::default();

        h.protocol_version = VarInt::from_stream(&mut stream)?;

        h.server_address = DataString::from_stream(&mut stream)?;

        let mut port = [0u8; 2];
        stream.read(&mut port)?;
        h.server_port |= (port[0] as u16) << 8;
        h.server_port |= port[1] as u16;

        h.next_state = VarInt::from_stream(&stream)?;

        Ok(h)
    }

    fn to_vec(&mut self) -> Vec<u8> {
        let mut v = Vec::<u8>::new();

        v.append(&mut self.protocol_version.to_vec());
        v.append(&mut self.server_address.to_vec());
        v.append(&mut self.server_port.to_le_bytes().to_vec());
        v.append(&mut self.next_state.to_vec());
        v
    }
}
