use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::handler::{Handler, HandlerMap};

pub trait Plugin: Send + Sync + std::fmt::Debug {
    type State: Default + Sized;

    // Returns the name of this plugin
    fn name() -> &'static str where Self: Sized;

    // Returns this plugin's API
    fn api() -> HandlerMap where Self: Sized;
}

#[derive(Default, Debug)]
pub struct Counter {}

impl Plugin for Counter {
    type State = usize;

    fn name() -> &'static str { "counter" }

    fn api() -> HandlerMap {
        let mut state = Arc::new(RwLock::new(0));
        let mut map = HashMap::new();
        // map.insert("incr", Handler::new(state.clone(), |t: Self::State| t + 1));
        map.insert("incr", Handler::new_nonsense());
        return map.into();
    }
}
