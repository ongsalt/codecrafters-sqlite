#[derive(Debug)]
pub struct Varint {
    pub value: i64,
    pub size: u8,
}

// from https://sqlite.org/src4/doc/trunk/www/varint.wiki

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

    // Wrong implementation
    // pub fn from_bytes(buf: &[u8]) -> Self {
    //     let a0 = buf[0];
    //     let (value, size) = match a0 {
    //         0..=240 => (a0.into(), 1),
    //         241..=248 => (240 + 256 * (u64::from(a0) - 241) + u64::from(buf[1]), 2),
    //         249 => (2288 + 256 * u64::from(buf[1]) + u64::from(buf[2]), 3),
    //         250 => (u64::from_be_bytes([0, 0, 0, 0, 0, buf[1], buf[2], buf[3]]), 4),
    //         251 => (u64::from_be_bytes([0, 0, 0, 0, buf[1], buf[2], buf[3], buf[4]]), 5),
    //         252 => (u64::from_be_bytes([0, 0, 0, buf[1], buf[2], buf[3], buf[4], buf[5]]), 6),
    //         253 => (u64::from_be_bytes([0, 0, buf[1], buf[2], buf[3], buf[4], buf[5], buf[6]]), 7),
    //         254 => (u64::from_be_bytes([0, buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]), 8),
    //         // 254 => (u64::from_be_bytes([0].iter().chain(&buf[0..7]).try_into().unwrap()), 1),
    //         255 => (u64::from_be_bytes(buf[0..8].try_into().unwrap()), 9),
    //     };
    //     Varint {
    //         value,
    //         size
    //     }
    // }
    pub fn to_bytes(&self) -> [u8; 9] {
        unimplemented!()
    }
}
