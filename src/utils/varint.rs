#[derive(Debug)]
pub struct Varint {
    pub value: i64,
    pub size: u8,
}


// from https://sqlite.org/src4/doc/trunk/www/varint.wiki
// Wrong one mf

impl Varint {
    pub fn from_bytes(buf: &[u8]) -> Self {
        let mut value: i64 = 0;
        let mut bytes_read: usize = 0;
        for (i, byte) in buf.iter().enumerate().take(9) {
            bytes_read += 1;
            if i == 8 {
                value = (value << 8) | *byte as i64;
                break;
            } else {
                value = (value << 7) | (*byte & 0b0111_1111) as i64;
                if *byte < 0b1000_0000 {
                    break;
                }
            }
        }
        Varint {
            value,
            size: bytes_read as u8,
        }
    }
    pub fn to_bytes(&self) -> [u8; 9] {
        unimplemented!()
    }
}
