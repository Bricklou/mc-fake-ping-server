use std::{fmt::Debug, net::TcpStream};

use crate::utils::varint::VarInt;

pub mod handshake;
pub mod packet;
pub mod status;

pub trait MCPacket: Default + Debug {
    fn from_stream(stream: &TcpStream) -> Result<Self, std::io::Error>;
    fn to_vec(&mut self) -> Vec<u8>;

    const ID: VarInt;
}
