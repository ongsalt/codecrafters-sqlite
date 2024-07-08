use anyhow::{bail, Result};
use nom::ToUsize;
use page::{parse_schema_table_page};
use std::fs::File;
use std::io::prelude::*;
use std::os::windows::fs::FileExt;
use std::vec;
use utils::DatabaseHeader;

mod btree;
mod page;
mod record;
mod utils;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    // Parse command and act accordingly
    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let mut file: File = File::open(&args[1])?;
            let mut header: [u8; 100] = [0; 100];
            file.read_exact(&mut header)?;

            let db_header = DatabaseHeader::from_bytes(&header);
            // The page size is stored at the 16th byte offset, using 2 bytes in big-endian order
            #[allow(unused_variables)]
            let page_size: u16 = u16::from_be_bytes([header[16], header[17]]);

            println!("database page size: {}", page_size);

            // table_schema is in the first page
            let mut first_page_buf: Vec<u8> = vec![0; page_size.to_usize()];
            file.seek_read(&mut first_page_buf, 0)
                .expect("this whould not failed here");

            // println!("[Header]: {:#?}", &db_header);
            
            let cells = parse_schema_table_page(&first_page_buf, &db_header);
            println!("number of tables: {}", cells.len());

        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
