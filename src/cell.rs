use crate::{
    page::{PageHeader, PageType},
    record::Record,
    utils::{DatabaseHeader, Varint},
};

#[derive(Debug)]
pub enum Cell {
    LeafTable {
        size: Varint,
        row_id: Varint,
        payload: Record,
        overflow_page: Option<u32>,
    },
    InteriorTable {
        left_child: u32, // Option<Box<Cell>>
        key: Varint,
    },
    LeafIndex {
        size: Varint,
        payload: Record,
        overflow_page: Option<u32>,
    },
    InteriorIndex {
        left_child: u32, // Option<Box<Cell>>
        size: Varint,
        key: Varint,
        overflow_page: Option<u32>,
    },
}

impl Cell {
    pub fn from_bytes(
        page_buf: &[u8],
        position: u64,
        page_header: &PageHeader,
        db_header: &DatabaseHeader,
    ) -> Result<Self, &'static str> {
        match page_header.kind {
            PageType::LeafTable => {
                let size = Varint::from_bytes(&page_buf[(position as usize)..]);
                let mut padding = size.size as usize;
                let row_id = Varint::from_bytes(&page_buf[(position as usize + padding)..]);
                padding += row_id.size as usize;
                let payload = Record::from_bytes(&page_buf, padding + position as usize, db_header)?;
                // println!("TODO: overflow page");
                Ok(Cell::LeafTable {
                    size,
                    payload,
                    row_id,
                    overflow_page: Option::None,
                })
            }
            PageType::InteriorTable => {
                unimplemented!()
            }
            PageType::LeafIndex => {
                unimplemented!()
            }
            PageType::InteriorIndex => {
                unimplemented!()
            }
        }
    }
}
