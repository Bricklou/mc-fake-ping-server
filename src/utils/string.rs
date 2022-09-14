use std::{io::Read, iter::FromIterator, net::TcpStream};

use super::varint::VarInt;

#[derive(Debug)]
pub struct DataString(String);

impl From<String> for DataString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Default for DataString {
    fn default() -> Self {
        Self(String::default())
    }
}

impl DataString {
    pub fn from_stream(mut stream: &TcpStream) -> Result<Self, std::io::Error> {
        let size: u32 = VarInt::from_stream(&stream)?.into();

        let mut s = vec![0u8; size as usize];
        stream.read_exact(&mut s)?;

        let s = s.iter().map(|b| *b as char).collect::<Vec<_>>();

        Ok(Self(String::from_iter(s)))
    }

    pub fn to_vec(&mut self) -> Vec<u8> {
        let mut v = Vec::<u8>::new();

        let mut size = VarInt::from(self.0.len() as u32).to_vec();
        v.append(&mut size);
        v.append(&mut self.0.as_bytes().to_vec());

        v
    }
}

impl Into<String> for DataString {
    fn into(self) -> String {
        self.0
    }
}
