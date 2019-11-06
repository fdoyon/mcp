use crate::{QTable, query};

pub enum ColumnStore {
    Timestamp(Vec<u64>),
    Symbol(Vec<String>),
    Float(Vec<f64>),
}

fn get_col_index(table: &mut QTable, name: &str) -> usize {
    unimplemented!()
}

fn extract_ts(table: &mut QTable, name: &str) -> Vec<u64> {
    unimplemented!()
}

fn extract_symbol(table: &mut QTable, name: &str) -> Vec<String> {
    unimplemented!()
}

fn extract_float(table: &mut QTable, name: &str) -> Vec<f64> {
    unimplemented!()
}

pub trait Schema {
    fn create_time_extractor(&self) -> Box<dyn Fn(&mut QTable) -> ColumnStore>;
    fn create_symbol_extractor(&self) -> Box<dyn Fn(&mut QTable) -> ColumnStore>;
    fn create_data_extractors(&self) -> Vec<Box<dyn Fn(&mut QTable) -> ColumnStore>>;
}

impl Schema for query::Query_Schema {
    fn create_time_extractor(&self) -> Box<dyn Fn(&mut QTable) -> ColumnStore> {
        let col = self.timestamp_col.clone();
        Box::new(move |table| ColumnStore::Symbol(extract_symbol(table, &col)))
    }

    fn create_symbol_extractor(&self) -> Box<dyn Fn(&mut QTable) -> ColumnStore> {
        let col = self.identifier_col.clone();
        Box::new(move |table| ColumnStore::Symbol(extract_symbol(table, &col)))
    }

    fn create_data_extractors(&self) -> Vec<Box<dyn Fn(&mut QTable) -> ColumnStore>> {
        let mut fns = Vec::<Box<dyn Fn(&mut QTable) -> ColumnStore>>::new();
        for field in self.data_columns.iter() {
            let col = self.name.clone();
            match field.kind {
                Kind::TIMESTAMP => fns.push(Box::new(move |table| {
                    ColumnStore::Timestamp(extract_ts(table, &col))
                })),
                Kind::SYMBOL => fns.push(Box::new(move |table| {
                    ColumnStore::Symbol(extract_symbol(table, &col))
                })),
                Kind::FLOAT => fns.push(Box::new(move |table| {
                    ColumnStore::Float(extract_float(table, &col))
                })),
            }
        }
        fns
    }
}
