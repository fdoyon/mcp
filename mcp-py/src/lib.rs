extern crate pyo3;
extern crate mcp_core;
extern crate protobuf;

use protobuf::Message;
use pyo3::prelude::*;
use std::os::raw::c_void;
use mcp_core::*;
use std::sync::{Arc, Mutex};
use std::intrinsics::uninit;

struct NameColumn {
    name: String,
    data: ColumnStore,
}


#[pyclass(module = "mcp")]
struct PollingMultiplexer {
    mcp: mcp_core::Multiplexer<mcp_core::PollingStrategy<Vec<NameColumn>>>,
    data: Arc<Mutex<Vec<NamedColumn>>>,
}

#[pymethods]
impl PollingMultiplexer {
    #[new]
    fn new(obj: &PyRawObject, query_proto: &[u8]) {
        let mut query = Query::new();
        query.merge_from_bytes(query_proto).expect("cannot decode query");

        let data = Arc::new(Mutex::new(Vec::new()));

        obj.init(PollingMultiplexer {
            mcp: Multiplexer::new(query, PollingStrategy::new(data.clone())),
            data,
        });
        unimplemented!("does not fit on a slide but you get the idea")
    }

    fn poll(&mut self, py: Python<'_>) -> PyResult<PyObject> {
        let mut data_lock = self.data.get_mut();
        let mut poll = Vec::new();
        ::std::mem::swap(&mut poll, &mut data_lock);
        wrap_dataframe(py, &mut poll)
    }
}

fn create_dataframe(py: Python<'_>) -> PyResult<PyObject> {
    unimplemented!()
}

fn add_numpy_column(py: Python<'_>, df: &mut PyObject, ptr_numpy: * c_void, len: usize) -> PyResult<()> {
    unimplemented!()
}

fn wrap_dataframe(py: Python<'_>, cols: Vec<NameColumn>) -> PyResult<PyObject> {
    let mut df = create_dataframe(py)?;

    for NameColumn { name, data } in cols.into_iter() {
        let (ptr_numpy, len) = match data {
            ColumnStore::Symbol(s) => (s.as_ptr() as * c_void, s.len()),
            ColumnStore::Timestamp(t) => (t.as_ptr() as * c_void, s.len()),
            ColumnStore::Float(f) => (f.as_ptr() as * c_void, s.len()),
        };

        add_numpy_column(py, &mut df, ptr_numpy, len)?;
    }

    Ok(df)
}

#[pymodule]
fn mcp(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PollingMultiplexer>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
