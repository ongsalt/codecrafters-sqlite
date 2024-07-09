use anyhow::{bail, Result};
use format::{SqliteFile, Table};
use itertools::Itertools;

mod sql_parser;
mod format;
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
                    Ok(tables) => println!("{:#?}", tables.iter().map(|it| &it.name).join(" ")),
                    Err(e) => println!("failed: {}", e),
                },
                Err(e) => println!("failed: {}", e),
            }
        }
        other => {
            let lenght = other.len();
            if lenght > 1 && other.starts_with('"') && other.ends_with('"') {
                // if it's "quoted" -> send to sql parser
                let mut file = SqliteFile::open(&args[1])?;

                &other[1..lenght - 2];
            } else {
                bail!("Missing or invalid command passed: {}", command);
            }
        }
    }

    Ok(())
}
