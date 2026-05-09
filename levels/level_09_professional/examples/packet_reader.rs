struct PacketReader<'a> {
    input: &'a [u8],
    offset: usize,
}

impl<'a> PacketReader<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self { input, offset: 0 }
    }

    fn remaining(&self) -> usize {
        self.input.len().saturating_sub(self.offset)
    }

    fn read_u8(&mut self) -> Option<u8> {
        if self.remaining() < 1 {
            return None;
        }

        let value = self.input[self.offset];
        self.offset += 1;
        Some(value)
    }

    fn read_u16_be(&mut self) -> Option<u16> {
        if self.remaining() < 2 {
            return None;
        }

        let high = self.input[self.offset] as u16;
        let low = self.input[self.offset + 1] as u16;
        self.offset += 2;

        Some((high << 8) | low)
    }

    fn read_bytes(&mut self, len: usize) -> Option<&'a [u8]> {
        if self.remaining() < len {
            return None;
        }

        let start = self.offset;
        self.offset += len;
        Some(&self.input[start..start + len])
    }
}

fn main() {
    let packet = [0x12, 0x34, 0x01, 0x04, b'r', b'u', b's', b't'];
    let mut reader = PacketReader::new(&packet);

    let id = reader.read_u16_be().expect("packet has id");
    let flags = reader.read_u8().expect("packet has flags");
    let payload_len = reader.read_u8().expect("packet has payload length") as usize;
    let payload = reader.read_bytes(payload_len).expect("packet has payload");
    let payload = std::str::from_utf8(payload).expect("payload is valid UTF-8");

    println!("id = {}", id);
    println!("flags = {}", flags);
    println!("payload = {}", payload);

    let mut short_reader = PacketReader::new(&[0x00]);
    if short_reader.read_u16_be().is_none() {
        println!("short packet rejected");
    }
}
