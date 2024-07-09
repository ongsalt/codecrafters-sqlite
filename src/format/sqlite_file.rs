use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

use anyhow::{Result, Error};
use nom::ToUsize;

use super::{table, DatabaseHeader, Page, Table};

pub struct SqliteFile {
    file: File,
    pub header: DatabaseHeader,
    pub tables: Vec<Table>
}

impl<'a> SqliteFile {
    pub fn open(path: &str) -> Result<Self> {
        let mut file = File::open(&path)?;

        let mut header_buf: [u8; 100] = [0; 100];
        file.read_exact(&mut header_buf)?;
        let header = DatabaseHeader::from_bytes(&header_buf);

        let mut buf: Vec<u8> = vec![0; header.page_size as usize];
        file.seek(SeekFrom::Start(0))?;
        file.read_exact(&mut buf)?;

        let tables = Table::from_page(
            &Page::from_bytes_with_padding(&buf, &header, 100)
        ).map_err(|e| Error::msg(e))?;

        Ok(Self {
            file,
            header,
            tables
        })
    }

    pub fn read_page(&mut self, page_number: u64) -> Result<Page> {
        let start = (page_number - 1) * self.header.page_size as u64;
        let size = self.header.page_size.to_usize();
        let padding = if page_number == 1 { 100 } else { 0 };

        let mut buf: Vec<u8> = vec![0; size];
        self.file.seek(SeekFrom::Start(start))?;
        self.file.read_exact(&mut buf)?;

        Ok(Page::from_bytes_with_padding(&buf, &self.header, padding))
    }

}
