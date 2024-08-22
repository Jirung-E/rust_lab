use std::mem::size_of;


#[repr(C, packed)]
#[derive(Debug, PartialEq, Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
pub struct PacketType(u8);
impl PacketType {
    pub const RAW: Self = Self(0);
    pub const MESSAGE: Self = Self(1);
    pub const MOVE: Self = Self(2);
    pub const ANIMATION: Self = Self(3);
    pub const UPDATE: Self = Self(4);
    pub const INIT: Self = Self(5);
}

pub type PacketSize = u16;


#[repr(C, packed)]
#[derive(Debug, PartialEq, Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
pub struct PacketHeader {
    size: PacketSize,
    packet_type: PacketType,
}


#[derive(Debug, PartialEq)]
pub struct RawPacket {
    header: PacketHeader,
    data: Vec<u8>,
}

impl RawPacket {
    pub fn new(packet_type: PacketType, data: &[u8]) -> Self {
        let size = (size_of::<PacketHeader>() + data.len()) as PacketSize;

        Self {
            header: PacketHeader {
                size,
                packet_type,
            },
            data: data.to_vec(),
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut data = bytemuck::bytes_of(&self.header).to_vec();
        data.extend_from_slice(&self.data);

        data
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        if data.len() < size_of::<PacketHeader>() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid data"));
        }

        let header = *bytemuck::from_bytes::<PacketHeader>(&data[0..size_of::<PacketHeader>()]);
        if data.len() < header.size as usize {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid data"));
        }

        let data = data[size_of::<PacketHeader>()..header.size as usize].to_vec();

        Ok(Self {
            header,
            data,
        })
    }
}


#[derive(Debug, PartialEq)]
pub struct MessagePacket {
    pub time: u128,
    pub msg: String,
}

impl MessagePacket {
    pub fn new(time: u128, msg: &str) -> Self {
        Self {
            time,
            msg: msg.to_string(),
        }
    }

    pub fn from_raw(raw: RawPacket) -> Result<Self, std::io::Error> {
        if raw.data().len() < size_of::<u128>() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid data"));
        }
        
        let time = *bytemuck::from_bytes::<u128>(&raw.data()[0..size_of::<u128>()]);
        let msg = String::from_utf8_lossy(&raw.data()[size_of::<u128>()..]);

        Ok(Self::new(time, &msg))
    }

    pub fn as_raw(&self) -> RawPacket {
        let mut data = bytemuck::bytes_of(&self.time).to_vec();
        data.extend_from_slice(self.msg.as_bytes());

        RawPacket::new(PacketType::MESSAGE, &data)
    }
}


















#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_packet() {
        let data = vec![1, 2, 3, 4, 5];
        let packet = RawPacket::new(PacketType::RAW, &data);

        let serialized = packet.as_bytes();
        assert_eq!(serialized, vec![8, 0, 0, 1, 2, 3, 4, 5]);   // little-endian

        let deserialized = RawPacket::from_bytes(&serialized).unwrap();
        assert_eq!(packet, deserialized);
    }

    #[test]
    fn test_message_packet() {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap()
            .as_millis();
        let msg = "Hello, World!";
        let packet = MessagePacket::new(time, msg);

        assert_eq!(packet.time, time);
        assert_eq!(packet.msg, msg);
    }

    #[test]
    fn test_message_packet_from_raw() {
        let time = 1234567890;
        let msg = "Hello, World!";
        let packet = MessagePacket::new(time, msg);
        let raw = packet.as_raw();

        let new_packet = MessagePacket::from_raw(raw).unwrap();
        assert_eq!(packet, new_packet);
    }
}