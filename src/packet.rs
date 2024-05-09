pub enum FieldType {
    U8(u8),
    U16(u16),
    U8Type,
    U16Type,
    Err,
}

impl FieldType {
    pub fn value(&self) -> u16 {
        match self {
            Self::U8(x) => {
                return *x as u16
            },
            Self::U16(x) => {
                return *x as u16
            },
            _ => {
                return 0
            }
        }
    }
}

pub struct RawPacket {
    raw: Vec<u8>,
    next: usize,
}

impl RawPacket {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            raw: Vec::new(),
            next: 0,
        }
    }

    pub fn from(raw: &[u8]) -> Self {
        Self {
            raw: raw.to_vec(),
            next: 0,
        }
    }

    pub fn next(&mut self, ty: FieldType) -> FieldType {
        match ty {
            FieldType::U8Type => {
                self.next += 1;
                return FieldType::U8(self.raw[self.next - 1] as u8)
            },
            FieldType::U16Type => {
                self.next += 2;
                let raw_field: &[u8; 2] = &[self.raw[self.next - 2], self.raw[self.next - 1]];
                return FieldType::U16(u16::from_le_bytes(*raw_field))
            },
            _ => {
                return FieldType::Err
            }
        }
    }
}

pub fn print_rawpacket(packet: &mut  RawPacket) {
    println!("Ethernet (");
    println!("\tDstMac: {:X}:{:X}:{:X}:{:X}:{:X}:{:X}", packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value());
    println!("\tSrcMac: {:X}:{:X}:{:X}:{:X}:{:X}:{:X}", packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value());
    let proto = packet.next(FieldType::U16Type).value();
    println!("\tProto: {}", proto);
    println!(")");

    if proto == 8 {
        println!("IP (");
        println!("\tVersionAndIHL: {}", packet.next(FieldType::U8Type).value());
        println!("\tTypeOfService: {}", packet.next(FieldType::U8Type).value());
        println!("\tTotalLength: {}", packet.next(FieldType::U16Type).value());
        println!("\tID: {}", packet.next(FieldType::U16Type).value());
        println!("\tFlags: {}", packet.next(FieldType::U16Type).value());
        println!("\tTTL: {}", packet.next(FieldType::U8Type).value());
        println!("\tProtocol: {}", packet.next(FieldType::U8Type).value());
        println!("\tCheckSum: {}", packet.next(FieldType::U16Type).value());
        println!("\tSrcIP: {}.{}.{}.{}", packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value());
        println!("\tDstIP: {}.{}.{}.{}", packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value(), packet.next(FieldType::U8Type).value());
        println!(")");
    }
}
