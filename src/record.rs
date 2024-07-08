use itertools::Itertools;

use crate::utils::{DatabaseHeader, TextEncoding, Varint};
#[derive(Debug)]
pub enum RecordSerial {
    Null,
    I8(i8),
    I16(i16),
    I24(i32),
    I32(i32),
    I48(i64),
    I64(i64),
    F64(f64),
    Zero,
    One,
    Reserved1,
    Reserved2,
    Blob(Vec<u8>),
    String(String),
}

#[derive(Debug)]
pub struct Record {
    pub header_size: Varint,
    pub content: Vec<RecordSerial>,
}

impl Record {
    pub fn from_bytes(
        buf: &[u8],
        position: usize,
        db_header: &DatabaseHeader,
    ) -> Result<Self, &'static str> {
        let lenght: usize = buf.len();
        let header_size: Varint = Varint::from_bytes(&buf[position..]);

        let mut header_current = position + header_size.size as usize;
        let mut current = position + header_size.value as usize;
        let mut content = Vec::<RecordSerial>::new();

        // Read header from {padding} to {header_size}
        while header_current < header_size.value as usize + position {
            let serial_code: Varint = Varint::from_bytes(&buf[header_current..]);
            header_current += serial_code.size as usize;
            let (consumed, record_serial) = match serial_code.value {
                0 => (0, RecordSerial::Null),
                1 => (1, RecordSerial::I8(i8::from_be_bytes([buf[current]]))),
                2 => (
                    2,
                    RecordSerial::I16(i16::from_be_bytes([buf[current], buf[current + 1]])),
                ),
                3 => (
                    3,
                    RecordSerial::I24(i32::from_be_bytes([
                        0,
                        buf[current],
                        buf[current + 1],
                        buf[current + 2],
                    ])),
                ),
                4 => (
                    4,
                    RecordSerial::I32(i32::from_be_bytes([
                        buf[current],
                        buf[current + 1],
                        buf[current + 2],
                        buf[current + 3],
                    ])),
                ),
                5 => (
                    6,
                    RecordSerial::I48(i64::from_be_bytes([
                        0,
                        0,
                        buf[current],
                        buf[current + 1],
                        buf[current + 2],
                        buf[current + 3],
                        buf[current + 4],
                        buf[current + 5],
                    ])),
                ),
                6 => (
                    8,
                    RecordSerial::I64(i64::from_be_bytes([
                        buf[current],
                        buf[current + 1],
                        buf[current + 2],
                        buf[current + 3],
                        buf[current + 4],
                        buf[current + 5],
                        buf[current + 6],
                        buf[current + 7],
                    ])),
                ),

                7 => (
                    8,
                    RecordSerial::F64(f64::from_be_bytes([
                        buf[current],
                        buf[current + 1],
                        buf[current + 2],
                        buf[current + 3],
                        buf[current + 4],
                        buf[current + 5],
                        buf[current + 6],
                        buf[current + 7],
                    ])),
                ),

                8 => (0, RecordSerial::Zero),
                9 => (0, RecordSerial::One),
                10 => (0, RecordSerial::Reserved1),
                11 => (0, RecordSerial::Reserved2),
                n if n % 2 == 0 => {
                    let size = (n - 12) / 2;
                    (
                        size,
                        RecordSerial::Blob(buf[current..(current + size as usize)].into()),
                    )
                }
                n => {
                    let size = (n - 13) / 2;
                    let bytes = buf[current..(current + size as usize)].to_vec();
                    match &db_header.text_encoding {
                        TextEncoding::UTF8 => match String::from_utf8(bytes) {
                            Result::Ok(string) => (size, RecordSerial::String(string)),
                            Result::Err(_) => return Err("Invalid utf8 string"),
                        },
                        TextEncoding::UTF16BE => {
                            let bytes = bytes
                                .iter()
                                .chunks(2)
                                .into_iter()
                                .map(|b| {
                                    let b: Vec<_> = b.collect();
                                    // might panic
                                    u16::from_be_bytes([*b[0], *b[1]])
                                })
                                .collect_vec();
                            match String::from_utf16(&bytes) {
                                Result::Ok(string) => (size, RecordSerial::String(string)),
                                Result::Err(_) => return Err("Invalid utf8 string"),
                            }
                        }
                        TextEncoding::UTF16LE => {
                            let bytes = bytes
                                .iter()
                                .chunks(2)
                                .into_iter()
                                .map(|b| {
                                    let b: Vec<_> = b.collect();
                                    // might panic
                                    u16::from_le_bytes([*b[0], *b[1]])
                                })
                                .collect_vec();
                            match String::from_utf16(&bytes) {
                                Result::Ok(string) => (size, RecordSerial::String(string)),
                                Result::Err(_) => return Err("Invalid utf8 string"),
                            }
                        }
                    }
                }
            };
            // println!("Record serial {:#?}", record_serial);
            current += consumed as usize;
            content.push(record_serial);
        }
        Ok(Record {
            header_size,
            content,
        })
    }
}
