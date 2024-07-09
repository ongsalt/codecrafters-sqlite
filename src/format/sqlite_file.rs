use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

use nom::ToUsize;

use super::{DatabaseHeader, Page};

pub struct SqliteFile {
    file: File,
    pub header: DatabaseHeader,
}

impl<'a> SqliteFile {
    pub fn open(path: &str) -> Result<Self, io::Error> {
        let mut file = File::open(&path)?;
        let mut header_buf: [u8; 100] = [0; 100];
        file.read_exact(&mut header_buf)?;
        file.seek(SeekFrom::Start(0)).unwrap();

        Ok(Self {
            file,
            header: DatabaseHeader::from_bytes(&header_buf),
        })
    }

    pub fn read_page(&mut self, page_number: u64) -> Result<Page, io::Error> {
        let start = (page_number - 1) * self.header.page_size as u64;
        let size = self.header.page_size.to_usize();
        let padding = if page_number == 1 { 100 } else { 0 };

        let mut buf: Vec<u8> = vec![0; size];
        self.file.seek(SeekFrom::Start(start))?;
        self.file.read_exact(&mut buf)?;

        Ok(Page::from_bytes_with_padding(&buf, &self.header, padding))
    }

}
