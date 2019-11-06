use crate::Schema;
use std::task::Poll;
use std::sync::{Mutex, Arc};

pub trait Strategy {
    type Target: Default + Sized;
    fn extract_data(&self, schema: &dyn Schema) -> Self::Target;
    fn publish(&mut self, aggregate: Self::Target);
    fn aggregate(
        &mut self,
        schema: &dyn Schema,
        aggregate: Self::Target,
        additions: Vec<Self::Target>,
    ) -> Self::Target;
}

pub struct PollingStrategy<T> {
    data: Arc<Mutex<T>>
}

impl<T: Default + Sized> PollingStrategy<T> {
    pub fn new(data: Arc<Mutex<T>>) -> Self {
        PollingStrategy { data }
    }
    pub fn poll() -> T {
        T::default()
    }
}

impl<T: Default + Sized> Strategy for PollingStrategy<T> {
    type Target = T;

    fn extract_data(&self, schema: &Schema) -> Self::Target {
        unimplemented!()
    }

    fn publish(&mut self, aggregate: Self::Target) {
        *self.data.get_mut() = aggregate
    }

    fn aggregate(&mut self, schema: &Schema, aggregate: Self::Target, additions: Vec<Self::Target>) -> Self::Target {
        unimplemented!()
    }
}