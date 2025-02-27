use itertools::Itertools;

use super::{
    cell::Cell,
    page::Page,
    record::{Record, RecordSerial},
};

#[derive(Debug)]
pub struct Table {
    pub kind: String,
    pub name: String,
    pub table_name: String,
    pub root_page: i64,
    pub sql: String,
}

impl Table {
    pub fn from_page(page: &Page) -> Result<Vec<Self>, &'static str> {
        page.cells
            .iter()
            .map(|it| Table::from_schema_cell(it))
            .try_collect()
    }
    pub fn from_schema_cell(cell: &Cell) -> Result<Self, &'static str> {
        match cell {
            Cell::LeafTable { payload, .. } => Table::from_schema_record(payload),
            _ => Err("Not a schema cell"),
        }
    }
    pub fn from_schema_record(record: &Record) -> Result<Self, &'static str> {
        if record.content.len() != 5 {
            return Err("Not a schema record");
        }

        let (kind, name, table_name, root_page, sql) = match &record.content[0..5] {
            [RecordSerial::String(kind), RecordSerial::String(name), RecordSerial::String(table_name), root_page, RecordSerial::String(sql)] =>
            {
                let root_page = match root_page {
                    RecordSerial::I8(i) => *i as i64,
                    RecordSerial::I16(i) => *i as i64,
                    RecordSerial::I24(i) => *i as i64,
                    RecordSerial::I32(i) => *i as i64,
                    RecordSerial::I48(i) => *i as i64,
                    RecordSerial::I64(i) => *i as i64,
                    _ => return Err("Not a table schema"),
                };

                (
                    kind.clone(),
                    name.clone(),
                    table_name.clone(),
                    root_page,
                    sql.clone(),
                )
            }
            _ => return Err("Not a table schema"),
        };

        Ok(Table {
            kind,
            name,
            table_name,
            root_page,
            sql,
        })
    }
}
