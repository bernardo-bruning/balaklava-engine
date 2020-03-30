use crate::backend::Handle;
use std::collections::HashMap;
use std::marker::PhantomData;

struct Pool<T> {
    sequencer: u64,
    records: HashMap<u64, T>
}

impl <T> Default for Pool<T> {
    fn default() -> Self {
        Self {
            sequencer: 0,
            records: HashMap::new()
        }
    }
}

impl <T> Pool<T> {
    fn insert(&mut self, record: T) -> Handle<T> {
        self.sequencer+= 1;
        self.records.insert(self.sequencer, record);
        Handle {
            identifier: self.sequencer,
            type_marker: PhantomData
        }
    }

    fn remove(&mut self, handle: &Handle<T>) {
        self.records.remove(&handle.identifier);
    }

    fn borrow(&self, handle: &Handle<T>) -> Option<&T> {
        self.records.get(&handle.identifier)
    }
}