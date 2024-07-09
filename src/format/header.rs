use std::fmt::Display;

#[derive(Debug)]
pub enum TextEncoding {
    UTF8 = 1,
    UTF16LE = 2,
    UTF16BE = 3,
}

#[derive(Debug)]
pub struct SQLiteVersion {
    pub x: u8,
    pub y: u8,
    pub z: u8
}

impl Display for SQLiteVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.x, self.y, self.z)
    }
}

impl SQLiteVersion {
    fn parse(buf: [u8; 4]) -> Self {
        let version = u32::from_be_bytes(buf);
        SQLiteVersion {
            x: (version / 1000000) as u8,
            y: ((version % 1000000) / 1000) as u8 ,
            z: (version % 1000) as u8
        }
    }
}

#[derive(Debug)]
pub struct DatabaseHeader {
    pub page_size: u16,
    pub write_version: u8,
    pub read_version: u8,
    pub page_reserved_bytes: u8, // most of the time 0,
    pub maximum_embedded_payload_fraction: u8,
    pub minimum_embedded_payload_fraction: u8,
    pub leaf_payload_fraction: u8, // = 32
    pub file_change_counter: u32,
    pub pages_count: u32,
    pub first_free_list_trunk: u32,
    pub free_list_count: u32,
    pub schema_cookie: u32,
    pub schema_format_number: u32,
    pub default_page_cache_size: u32,
    pub largest_root_btree_page: u32,
    pub text_encoding: TextEncoding,
    pub user_version: u32,
    pub incremental_vacuum_mode: bool, // 4 bytes
    pub application_id: u32,
    // reserved 72+20
    pub version_valid_for: u32,
    pub sqlite_version: SQLiteVersion,
}

impl DatabaseHeader {
    pub fn from_bytes(buf: &[u8; 100]) -> Self {
        // TODO: im lazy
        DatabaseHeader {
            page_size: u16::from_be_bytes([buf[16], buf[17]]),
            write_version: buf[18],
            read_version: buf[19],
            page_reserved_bytes: buf[20], // most of the time 0,
            maximum_embedded_payload_fraction: buf[21],
            minimum_embedded_payload_fraction: buf[22],
            leaf_payload_fraction: buf[23], // = 32
            file_change_counter: u32::from_be_bytes([buf[24], buf[25], buf[26], buf[27]]),
            pages_count: u32::from_be_bytes([buf[28], buf[29], buf[30], buf[31]]),
            // first_free_list_trunk: buf[0..8].try_into().unwrap(),
            first_free_list_trunk: 0,
            free_list_count: 0,
            schema_cookie: 0,
            schema_format_number: 0,
            default_page_cache_size: 0,
            largest_root_btree_page: 0,
            text_encoding: match u32::from_be_bytes([buf[56], buf[57], buf[58], buf[59]]) {
                1 => TextEncoding::UTF8,
                2 => TextEncoding::UTF16LE,
                3 => TextEncoding::UTF16BE,
                n => {
                    println!("Found text_encoding:{} Fallback to utf8 (or should i panic)", n);
                    TextEncoding::UTF8
                }
            },
            user_version: 0,
            incremental_vacuum_mode: false, // 4 bytes
            application_id: 0,
            // reserved 72+20
            version_valid_for: 0,
            sqlite_version: SQLiteVersion::parse(buf[96..100].try_into().unwrap()),
        
        }
    }
}