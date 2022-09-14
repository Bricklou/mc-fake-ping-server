use std::{io::Write, net::TcpStream};

use crate::utils::varint::VarInt;

use super::MCPacket;

#[derive(Debug)]
pub struct Packet<T> {
    pub paquet_id: VarInt,
    pub data: T,
}

impl<T> Default for Packet<T>
where
    T: MCPacket,
{
    fn default() -> Self {
        Self {
            paquet_id: 0.into(),
            data: T::default(),
        }
    }
}

impl<T> Packet<T>
where
    T: MCPacket,
{
    pub fn from_stream(mut stream: &TcpStream) -> Result<Packet<T>, std::io::Error> {
        let mut p = Packet::<T>::default();

        VarInt::from_stream(&stream)?;

        p.paquet_id = VarInt::from_stream(&mut stream)?;
        p.data = T::from_stream(stream)?;

        Ok(p)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let v = Vec::<u8>::new();
        v
    }

    pub fn send(&mut self, mut stream: &TcpStream) -> Result<(), std::io::Error> {
        let mut v = Vec::<u8>::new();

        v.append(&mut self.paquet_id.to_vec());
        v.append(&mut self.data.to_vec());
        let mut s = VarInt::from(v.len() as u32).to_vec();
        s.append(&mut v);

        stream.write_all(&mut s)?;

        Ok(())
    }
}

impl<T> From<T> for Packet<T>
where
    T: MCPacket,
{
    fn from(mc_paquet: T) -> Self {
        let mut p: Packet<T> = Packet::<T>::default();

        p.data = mc_paquet;
        p.paquet_id = T::ID;

        p
    }
}
