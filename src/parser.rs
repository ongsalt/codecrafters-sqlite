use std::vec;

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::format::{Cell, SqliteFile};

// im not going to implement a full sql parser
pub struct Where {
    column_name: String,
    value: String,
}

pub enum Command {
    Count {
        table_name: String,
        conditions: Vec<Where>,
    },
    Select {
        table_name: String,
        column_names: Vec<String>,
        conditions: Vec<Where>,
    },
    SelectAll {
        table_name: String,
        conditions: Vec<Where>,
    },
}

peg::parser! {
    grammar sql_command() for str {
        pub rule command() -> Command
            = c:count() / c:select() / c:select_all() {c}
        pub rule count() -> Command
            = "SELECT" _ "COUNT(*)" _ "FROM" _ table_name:name() { Command::Count { table_name, conditions: vec![] } }
        pub rule select() -> Command
            = "SELECT" _ column_names:(name() ** (_* "," _*)) _ "FROM" _ table_name:name() { Command::Select { table_name, column_names, conditions: vec![] } }
        pub rule select_all() -> Command
            = "SELECT" _ "*" _ "FROM" _ table_name:name() { Command::SelectAll { table_name, conditions: vec![] } }
        pub rule create_table() -> Vec<String>
            = "CREATE" _ "TABLE" _ name() _ "(" _ column_names:((c:name() (_ name())* {c})** (_* "," _*)) _ ")" _* { column_names }

        rule _()
            = [' ' | '\n' | '\t']+
        // No condition nesting
        rule where_condition() -> Where
            = "WHERE" _ column_name:name() _ "=" _ value:name() { Where { column_name, value }}
        // there's a lot more rule for valid name
        rule name() -> String
            = name:$(['a'..='z' | '_']+) { name.to_string() }
    }
}

pub fn execute(command: &str, db: &mut SqliteFile) -> Result<()> {
    // why did i do this
    let command = sql_command::command(command)?;
    match command {
        Command::Count {
            table_name,
            conditions,
        } => {
            let table = db
                .tables
                .iter()
                .find(|it| it.name == table_name)
                .ok_or_else(|| anyhow!("no table named {}", table_name))?;

            let page = db.read_page(table.root_page as u64)?;
            println!("{}", page.cells.len());
        }
        Command::SelectAll {
            table_name,
            conditions,
        } => {
            let table = db
                .tables
                .iter()
                .find(|it| it.name == table_name)
                .ok_or_else(|| anyhow!("no table named {}", table_name))?;
            // println!("{}", table.sql);
            let db_column_names = sql_command::create_table(&table.sql)?;

            let page = db.read_page(table.root_page as u64)?;
            let output = page
                .cells
                .iter()
                // .filter()
                .map(|cell| match cell {
                    Cell::LeafTable { payload, .. } => {
                        payload.content.iter().map(|it| it.to_string()).join("|")
                    }
                    _ => todo!(),
                })
                .join("\n");
            println!("{output}")
        }

        Command::Select {
            table_name,
            column_names,
            conditions,
        } => {
            let table = db
                .tables
                .iter()
                .find(|it| it.name == table_name)
                .ok_or_else(|| anyhow!("no table named {}", table_name))?;
            // println!("{}", table.sql);
            let db_column_names = sql_command::create_table(&table.sql)?;

            let column_indexes: Vec<_> = column_names
                .iter()
                .map(|name| {
                    db_column_names
                        .iter()
                        .find_position(|it| *it == name)
                        .map(|(i, _)| i)
                        .ok_or(anyhow!("column {name} not exists"))
                })
                .try_collect()?;

            let page = db.read_page(table.root_page as u64)?;
            let output = page
                .cells
                .iter()
                // .filter()
                .map(|cell| match cell {
                    Cell::LeafTable { payload, .. } => column_indexes
                        .iter()
                        .map(|i| payload.content[*i].to_string())
                        .join("|"),
                    _ => todo!(),
                })
                .join("\n");

            println!("{output}")
        }
    }

    Ok(())
}
