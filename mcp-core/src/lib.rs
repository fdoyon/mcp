extern crate chrono;
extern crate itertools;
extern crate protobuf;
extern crate rkdb;

use chrono::{Date, DateTime, Utc};
use itertools::Itertools;
use std::sync::mpsc::{channel, Sender};

mod interop;
mod strategies;
mod schema;

pub use crate::schema::*;
pub use crate::interop::*;
pub use crate::strategies::*;

use rkdb::types::U;
use std::intrinsics::likely;
use std::marker::PhantomData;
use std::sync::Arc;
use std::thread::spawn;

pub type QTable = rkdb::kbindings::KVal<'static>;

pub struct Multiplexer<T: Strategy> {
    sync: Sender<()>,
    strategy: PhantomData<T>,
}

impl<T: Strategy> Multiplexer<T> {
    pub fn new(query: Query,mut strategy:T) -> Self {
        let (tx, rx) = channel();

        // storage strategy
        // spawn the aggregate thread
        spawn(move || {
            let schema = query.schema.clone().unwrap();
            let mut data = Default::default();
            let (mut hist, mut live) = Self::build_workers(&query);
            if let Err(_) = rx.recv() {
                return;
            } // wait for the multiplexer to call Run
            {
                // initial snapshot
                let mut snapshots = Vec::<T::Target>::new();
                // gather snapshots for historical workers
                snapshots.extend(hist.iter_mut().map(|h| h.drain()));

                // collect live data
                snapshots.extend(live.iter_mut().map(|l| l.drain()));

                // send the first snapshot
                let send = strategy.aggregate(&schema, data, snapshots);

                ::std::mem::drop(hist); // we are done with the historical workers
                strategy.publish(send);
            }
            // keep the live threads running
            if live.len() == 0 {
                return;
            }
            while let Ok(_) = rx.recv() {}
        });

        // join against live queries
        // update strategy
        Multiplexer {
            sync: tx,
            strategy: PhantomData,
        }
    }

    //
    pub fn start(&self) -> bool {
        match self.sync.send(()) {
            Ok(_) => true,
            _ => false,
        }
    }

    fn build_workers(query: &Query) -> (Vec<HistoricalWorker<T>>, Vec<LiveWorker<T>>) {
        // build the historical query
        let mut historical_queries = Default::default();
        let mut live_queries = Default::default();

        let rdb_cutoff = Utc::now().date();

        for (srv, instruments) in query.get_instruments_by_server() {
            // hdb and rdb queries
            let (hdb, rdb) = create_historical(
                rdb_cutoff,
                query.schema.get_ref(),
                srv,
                instruments.get_codes(),
                query.start.as_ref(),
                query.end.as_ref(),
            );

            // tp
        }

        (historical_queries, live_queries)
    }
}

enum QueryType {
    HDB(Date<Utc>),
    RDB(Date<Utc>),
}

fn create_query<T: Strategy>(
    query_type: QueryType,
    schema: &Query_Schema,
    srv: &str,
    instruments: &[String],
    start: &Query_TimeReference,
    end: &Query_TimeReference,
) -> String {
    let condition = match QueryType {
        QueryType::HDB(last_day) => {}
        QueryType::RDB(current_day) => {}
    };

    let mut selected = vec![schema.timestamp_col.copy(), schema.identifier_col.copy()];
    selected.extend(
        schema
            .data_columns
            .iter()
            .map(|Query_Schema_Field { name, .. }| name.into()),
    );
    let selected = selected.join(",");

    return format!("$[`{},(),0b,({})]", schema.name, condition, selected);
}

fn create_historical<T: Strategy>(
    rdb_cutoff: Date<Utc>,
    schema: &Query_Schema,
    srv: &str,
    instruments: &[String],
    start: Option<&Query_TimeReference>,
    end: Option<&Query_TimeReference>,
) -> Vec<HistoricalWorker<T>> {
    let mut workers = Vec::new();

    if let Some(query::Query_TimeReference_oneof_is::epoch_ns(start_ns)) = start.is() {}
}

trait Worker<T: Strategy> {
    fn drain(&mut self) -> T::Target;
}

struct HistoricalWorker<T: Strategy> {
    data: Option<T::Target>,
}

impl<T: Strategy> Worker<T> for HistoricalWorker<T> {
    fn drain(&mut self) -> <T as Strategy>::Target {
        unimplemented!()
    }
}

struct LiveWorker<T: Strategy> {
    is_live: bool,
    data: T::Target,
}

impl<T: Strategy> Worker<T> for LiveWorker<T> {
    fn drain(&mut self) -> <T as Strategy>::Target {
        if self.is_live {
            self.is_live = false
        }
        let mut ret = Default::default();
        ::std::mem::swap(&mut ret, &mut self.data);
        ret
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
