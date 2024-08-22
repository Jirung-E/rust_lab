use std::collections::VecDeque;
use std::mem::size_of;

use super::protocol::*;


#[derive(Debug, PartialEq)]
pub enum Packet {
    Complete(RawPacket),
    Incomplete(Vec<u8>),
}


use Packet::*;


/// 뭉쳐온 패킷 분리 및 잘린 패킷 이어붙이기를 수행하는 큐 형태의 Parser
pub struct PacketParser {
    queue: VecDeque<Packet>,
}

impl PacketParser {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, data: &[u8]) {
        if data.len() == 0 {
            return;
        }

        let mut data = Vec::from(data);

        if let Some(Incomplete(prev)) = self.queue.back() {
            data.splice(0..0, prev.iter().cloned());
            self.queue.pop_back().unwrap();
        }

        let mut len = data.len();
        while len > 0 {
            if len < 2 {
                self.queue.push_back(Incomplete(data));
                break;
            }

            let size = *bytemuck::from_bytes::<PacketSize>(&data[0..size_of::<PacketSize>()]) as usize;
            
            if len < size {
                self.queue.push_back(Incomplete(data));
                break;
            }

            let packet = data.drain(0..size).collect::<Vec<u8>>();
            
            len = data.len();

            let packet = match RawPacket::from_bytes(&packet) {
                Ok(packet) => Complete(packet),
                Err(_) => continue,     // 알 수 없는 패킷은 버림
            };

            self.queue.push_back(packet);
        }
    }

    /// 한개 남았을 때 Incomplete이면 아직 완성 안된것이므로 pop하지 않음.  
    /// 두개 이상 남았을때 제일 앞 패킷이 Incomplete이면 모종의 이유로 완성 안된것이므로 값을 버리기 위해 pop.  
    pub fn pop(&mut self) -> Option<RawPacket> {
        if self.queue.len() == 1 {
            if let Some(Incomplete(_)) = self.queue.front() {
                return None;
            }
        }

        match self.queue.pop_front() {
            Some(Complete(some)) => Some(some),
            _ => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn front(&self) -> Option<&Packet> {
        self.queue.front()
    }

    pub fn back(&self) -> Option<&Packet> {
        self.queue.back()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<Packet> {
        self.queue.iter()
    }
}
























#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut parser = PacketParser::new();

        let packet = RawPacket::new(PacketType::RAW, b"update");
        parser.push(&packet.as_bytes());
        assert_eq!(parser.pop(), Some(packet));

        let packet = RawPacket::new(PacketType::RAW, b"remove");
        parser.push(&packet.as_bytes());
        assert_eq!(parser.pop(), Some(packet));

        let packet = RawPacket::new(PacketType::RAW, b"init 3 2 5 6");
        parser.push(&packet.as_bytes());
        assert_eq!(parser.pop(), Some(packet));
    }

    #[test]
    fn test_sliced_packet() {
        let mut parser = PacketParser::new();

        {
            let packet = RawPacket::new(PacketType::RAW, b"update");
            let bytes = packet.as_bytes();
            parser.push(&bytes[..3]);
            assert_eq!(parser.iter().last(), Some(&Incomplete(bytes[..3].to_vec())));
            assert_eq!(parser.pop(), None);

            parser.push(&bytes[3..]);
            assert_eq!(parser.iter().last(), Some(&Complete(packet)));
            // assert_eq!(parser.pop(), Some(packet));
            parser.pop();
        }

        {
            let packet = RawPacket::new(PacketType::RAW, b"remove");
            let bytes = packet.as_bytes();
            parser.push(&bytes[..6]);
            assert_eq!(parser.iter().last(), Some(&Incomplete(bytes[..6].to_vec())));
            assert_eq!(parser.pop(), None);

            parser.push(&bytes[6..]);
            assert_eq!(parser.iter().last(), Some(&Complete(packet)));
            // assert_eq!(parser.pop(), Some(packet));
            parser.pop();
        }

        {
            let packet1 = RawPacket::new(PacketType::RAW, b"init 3 2 5 6");
            let packet2 = RawPacket::new(PacketType::RAW, b"update");
            let packet3 = RawPacket::new(PacketType::RAW, b"update");
            let packet4 = RawPacket::new(PacketType::RAW, b"remove");
            let bytes1 = packet1.as_bytes();
            let bytes2 = packet2.as_bytes();
            let bytes3 = packet3.as_bytes();
            let bytes4 = packet4.as_bytes();

            let chained = bytes1.iter()
                .chain(bytes2.iter())
                .chain(bytes3.iter())
                .chain(bytes4.iter())
                .cloned()
                .collect::<Vec<u8>>();
            let cut = bytes1.len() + bytes2.len() + bytes3.len() + bytes4.len() / 2;

            parser.push(&chained[..cut]);
            assert_eq!(parser.iter().last(), Some(&Incomplete(bytes4[..bytes4.len() / 2].to_vec())));

            let mut quess = vec![
                Complete(packet1),
                Complete(packet2),
                Complete(packet3),
                Incomplete(bytes4[..bytes4.len() / 2].to_vec()),
            ];

            for it in parser.iter().zip(quess.iter()) {
                assert_eq!(it.0, it.1);
            }

            parser.push(&chained[cut..]);
            quess[3] = Complete(packet4);

            for it in parser.iter().zip(quess.iter()) {
                assert_eq!(it.0, it.1);
            }
        }
    }

    #[test]
    fn test_empty_packet() {
        let mut parser = PacketParser::new();

        parser.push(b"");
        assert_eq!(parser.len(), 0);

        parser.push(b"\n");
        assert_eq!(parser.len(), 1);
        assert_eq!(parser.pop(), None);
    }
}
