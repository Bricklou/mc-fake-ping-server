use serde::Serialize;

use crate::utils::{string::DataString, varint::VarInt};

use super::MCPacket;

#[derive(Serialize, Default, Debug)]
struct Version {
    name: String,
    protocol: u32,
}

#[derive(Serialize, Default, Debug)]
struct PlayerSample {
    name: String,
    id: String,
}

#[derive(Serialize, Default, Debug)]
struct Description {
    text: String,
}

#[derive(Serialize, Default, Debug)]
struct Players {
    max: u32,
    online: u32,
    sample: Vec<PlayerSample>,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    version: Version,
    players: Players,
    description: Description,
    favicon: String,
    preview_chat: bool,
    enforces_secure_chat: bool,
}

impl StatusResponse {
    pub fn default_fake() -> Self {
        Self {
            version: Version {
                name: "Custom rust version".to_string(),
                protocol: 760,
            },
            players: Players {
                max: 1000,
                online: 4,
                sample: Vec::new(),
            },
            description: Description {
                text: "I'm a proxy server in rust".to_string(),
            },
            favicon: "".to_string(),
            enforces_secure_chat: false,
            preview_chat: false,
        }
    }
}

impl MCPacket for StatusResponse {
    fn to_vec(&mut self) -> Vec<u8> {
        let mut v = Vec::<u8>::new();

        let mut str: DataString = serde_json::to_string(self).unwrap().into();
        v.append(&mut str.to_vec());

        v
    }

    fn from_stream(_stream: &std::net::TcpStream) -> Result<Self, std::io::Error> {
        todo!();
    }

    const ID: VarInt = VarInt::ZERO;
}
