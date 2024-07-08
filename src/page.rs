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

impl PageHeader {
    pub fn from_bytes(header_buf: &[u8]) -> Self {
        let kind: PageType = match &header_buf[0] {
            0x02 => PageType::InteriorIndex,
            0x05 => PageType::InteriorTable,
            0x0a => PageType::LeafIndex,
            0x0d => PageType::LeafTable,
            _ => panic!("Invalid Sqlite"),
        };

        let first_freeblock = u16::from_be_bytes([header_buf[1], header_buf[2]]);
        let number_of_cells = u16::from_be_bytes([header_buf[3], header_buf[4]]);
        let first_cell_content = u16::from_be_bytes([header_buf[5], header_buf[6]]);
        let fragmented_free_bytes = header_buf[7];
        let page_number = match kind {
            PageType::InteriorIndex | PageType::InteriorTable => Some(u32::from_be_bytes([
                header_buf[8],
                header_buf[9],
                header_buf[10],
                header_buf[11],
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

pub fn parse_schema_table_page(buf: &[u8], db_header: &DatabaseHeader) -> Vec<Cell> {
    let header = PageHeader::from_bytes(&buf[100..]);
    let cell_pointer_array: Vec<u16> = parse_cell_pointer_array(&buf, &header, 100);
    // println!("Page header: {:#?}", &header);
    // println!("cell_pointer_array: {:#?}", cell_pointer_array);

    let rows = cell_pointer_array.iter().map(|cell| Cell::from_bytes(&buf, cell.to_owned().into(), &header, db_header).unwrap()).collect();
    rows
}

pub fn parse_cell_pointer_array(buf: &[u8], header: &PageHeader, page_padding: usize) -> Vec<u16> {
    let padding = page_padding + match &header.kind {
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
