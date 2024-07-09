use anyhow::{bail, Result};
use format::{SqliteFile, Table};
use itertools::Itertools;
use parser::execute;

mod format;
mod parser;
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
            let mut file = SqliteFile::open(&args[1])?;

            println!("page size: {}", &file.header.page_size);

            match file.read_page(1) {
                Ok(page) => match Table::from_page(&page) {
                    Ok(tables) => println!("number of tables: {}", tables.len()),
                    Err(e) => println!("failed: {}", e),
                },
                Err(e) => println!("failed: {}", e),
            }
        }
        ".tables" => {
            let mut file = SqliteFile::open(&args[1])?;

            match file.read_page(1) {
                Ok(page) => match Table::from_page(&page) {
                    Ok(tables) => println!(
                        "{}",
                        tables
                            .iter()
                            .map(|it| &it.name)
                            .filter(|name| *name != "sqlite_sequence")
                            .join(" ")
                    ),
                    // Ok(tables) => println!("{:#?}", tables),
                    Err(e) => println!("failed: {}", e),
                },
                Err(e) => println!("failed: {}", e),
            }
        }
        other => {
            let lenght = other.len();
            if lenght > 1 {
                // if it's "quoted" -> send to sql parser
                let mut db = SqliteFile::open(&args[1])?;
                execute(&other, &mut db)?;
            } else {
                bail!("Missing or invalid command passed: {}", command);
            }
        }
    }

    Ok(())
}
