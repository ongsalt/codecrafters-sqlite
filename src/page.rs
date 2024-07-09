use crate::{cell::Cell, utils::DatabaseHeader};

#[derive(Debug)]
pub enum PageType {
    InteriorIndex = 0x02,
    InteriorTable = 0x05,
    LeafIndex = 0x0a,
    LeafTable = 0x0d,
}

#[derive(Debug)]
pub struct PageHeader {
    pub kind: PageType,
    // pub isFirstPage: bool,
    pub first_freeblock: u16,
    pub number_of_cells: u16,
    pub first_cell_content: u16,
    pub fragmented_free_bytes: u8,
    pub page_number: Option<u32>,
}

pub struct Page {
    pub header: PageHeader,
    pub cell_pointers: Vec<u16>,
     // content: &'a [u8],
    // pub raw: &'a [u8],
    pub cells: Vec<Cell>,
}

impl Page {
    pub fn from_bytes(buf: &[u8], db_header: &DatabaseHeader) -> Page {
        Page::from_bytes_with_padding(&buf, db_header, 0)
    }

    pub fn from_bytes_with_padding(
        buf: &[u8],
        db_header: &DatabaseHeader,
        padding: usize,
    ) -> Page {
        let header = PageHeader::from_bytes(&buf[padding..]);
        let cell_pointers = Page::parse_cell_pointer_array(&buf, &header, padding);
        let cells = cell_pointers
            .iter()
            .map(|cell: &u16| {
                Cell::from_bytes(&buf, cell.to_owned().into(), &header, db_header).unwrap()
            })
            .collect();

        Page {
            header,
            cell_pointers,
            // raw: &buf,
            cells
        }
    }

    fn parse_cell_pointer_array(buf: &[u8], header: &PageHeader, page_padding: usize) -> Vec<u16> {
        let padding = page_padding
            + match &header.kind {
                PageType::InteriorIndex | &PageType::InteriorTable => 12,
                _ => 8,
            };

        let mut out = Vec::<u16>::new();
        for i in 0..(header.number_of_cells as usize) {
            out.push(u16::from_be_bytes([
                buf[i * 2 + padding],
                buf[i * 2 + 1 + padding],
            ]));
        }
        out
    }
}

impl PageHeader {
    pub fn from_bytes(page_buf: &[u8]) -> Self {
        let kind: PageType = match &page_buf[0] {
            0x02 => PageType::InteriorIndex,
            0x05 => PageType::InteriorTable,
            0x0a => PageType::LeafIndex,
            0x0d => PageType::LeafTable,
            _ => panic!("Invalid Sqlite"),
        };

        let first_freeblock = u16::from_be_bytes([page_buf[1], page_buf[2]]);
        let number_of_cells = u16::from_be_bytes([page_buf[3], page_buf[4]]);
        let first_cell_content = u16::from_be_bytes([page_buf[5], page_buf[6]]);
        let fragmented_free_bytes = page_buf[7];
        let page_number = match kind {
            PageType::InteriorIndex | PageType::InteriorTable => Some(u32::from_be_bytes([
                page_buf[8],
                page_buf[9],
                page_buf[10],
                page_buf[11],
            ])),
            _ => None,
        };

        PageHeader {
            kind,
            first_freeblock,
            number_of_cells,
            first_cell_content,
            fragmented_free_bytes,
            page_number,
        }
    }
}
