use std::{
    fmt::Debug,
    io::{ErrorKind, Read},
    net::TcpStream,
};

const SEGMENT_BIT: u8 = 0b0111_1111;
const CONTINUE_BIT: u8 = 0b1000_0000;

pub struct VarInt([u8; 5]);

impl Default for VarInt {
    fn default() -> Self {
        Self::from([0u8; 5])
    }
}

impl Debug for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: i32 = Self::from(self.0).into();
        write!(f, "VarInt({})", v)
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        // Save value into a temporary variable
        let mut val = value;

        let mut buf = [0; 5];
        let mut i = 0;

        while val != 0 {
            buf[i] = (val & (SEGMENT_BIT as i32)) as u8;
            val = (val >> 7) & (i32::max_value() >> 6);
            if val != 0 {
                buf[i] |= CONTINUE_BIT;
            }
            i += 1;
        }

        Self(buf)
    }
}

impl From<u32> for VarInt {
    fn from(value: u32) -> Self {
        // Save value into a temporary variable
        let mut val = value;

        let mut buf = [0; 5];
        let mut i = 0;

        while val != 0 {
            buf[i] = (val & (SEGMENT_BIT as u32)) as u8;
            val = (val >> 7) & (u32::max_value() >> 6);
            if val != 0 {
                buf[i] |= CONTINUE_BIT;
            }
            i += 1;
        }

        Self(buf)
    }
}

impl From<[u8; 5]> for VarInt {
    fn from(value: [u8; 5]) -> Self {
        Self(value)
    }
}

impl Into<Vec<u8>> for VarInt {
    fn into(mut self) -> Vec<u8> {
        self.to_vec()
    }
}

impl VarInt {
    pub const ZERO: VarInt = VarInt([0, 0, 0, 0, 0]);

    pub fn from_stream(mut stream: &TcpStream) -> Result<Self, std::io::Error> {
        let mut d = [0 as u8; 5];
        let mut b = [CONTINUE_BIT as u8; 1];
        let mut index = 0;

        while (b[0] & CONTINUE_BIT) != 0 && index < 5 {
            stream.read(&mut b)?;
            d[index] = b[0];
            index += 1;
        }

        if (b[0] & CONTINUE_BIT) != 0 {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Invalid VarInt provided",
            ));
        }

        Ok(Self::from(d))
    }

    pub fn to_vec(&mut self) -> Vec<u8> {
        let mut v = Vec::<u8>::new();
        for i in self.0 {
            if i == 0 {
                break;
            }
            v.push(i);
        }
        if v.is_empty() {
            v.push(0);
        }
        v
    }
}

impl Into<i32> for VarInt {
    fn into(self) -> i32 {
        // Final value
        let mut value: i32 = 0;
        // Position into the byte array
        let mut position: u16 = 0;

        for current_byte in self.0 {
            // filter `current_byte` to keep only the least significants bytes
            // then save them into the value with an offset of `position`
            value |= ((current_byte & SEGMENT_BIT) as i32) << position;

            // if the most significant bit is 0, this means there is no more
            // byte to parse
            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;
        }

        value
    }
}

impl Into<u32> for VarInt {
    fn into(self) -> u32 {
        // Final value
        let mut value: u32 = 0;
        // Position into the byte array
        let mut position: u16 = 0;

        for current_byte in self.0 {
            // filter `current_byte` to keep only the least significants bytes
            // then save them into the value with an offset of `position`
            value |= ((current_byte & SEGMENT_BIT) as u32) << position;

            // if the most significant bit is 0, this means there is no more
            // byte to parse
            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use std::i32;

    use super::VarInt;

    #[test]
    fn int_to_varint() {
        let mut arr = [0; 5];

        let v = VarInt::from(255);
        arr[0..2].copy_from_slice(&[0xff, 0x01]); // 255

        assert_eq!(v.0, arr);

        let v = VarInt::from(-1);
        arr[0..5].copy_from_slice(&[0xff, 0xff, 0xff, 0xff, 0x0f]); // -1
        assert_eq!(v.0, arr);
    }

    #[test]
    fn varint_to_int() {
        let v = VarInt::from([0xff, 0x01, 0x00, 0x00, 0x00]); // 255

        let i: i32 = v.into();
        assert_eq!(i, 255);

        let v = VarInt::from([0xff, 0xff, 0xff, 0xff, 0x0f]); // -1

        let i: i32 = v.into();
        assert_eq!(i, -1);
    }
}
