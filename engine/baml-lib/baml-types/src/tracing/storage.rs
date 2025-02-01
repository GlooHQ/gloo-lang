use std::{collections::HashMap, sync::Arc};

use super::events::{FunctionId, LogEvent};

use tokio::sync::Mutex;

#[derive(Default)]
pub struct TraceStorage {
    // This is a lookup of event -> data (not event -> event)
    // For that you need to do multiple lookups
    span_map: HashMap<FunctionId, Arc<Mutex<Vec<Arc<LogEvent>>>>>,

    // usage_count
    usage_count: HashMap<FunctionId, usize>,
}

impl TraceStorage {
    pub fn get(&self, span_id: FunctionId) -> Option<Arc<Mutex<Vec<Arc<LogEvent>>>>> {
        self.span_map.get(&span_id).cloned()
    }

    pub fn put(&mut self, event: Arc<LogEvent>) {
        let span_id = event.span_id.clone();
        self.span_map
            .entry(span_id)
            .or_insert(Arc::new(Mutex::new(vec![])));

        let rt = tokio::runtime::Runtime::new().unwrap();
        if let Some(mutex) = self.span_map.get_mut(&event.span_id) {
            let mut events = rt.block_on(mutex.lock());
            events.push(event);
        }
    }
}
