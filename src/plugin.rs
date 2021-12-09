use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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
        let mut map = HashMap::new();
        // map.insert("incr", Handler::new(state.clone(), |t: Self::State| t + 1));
        map.insert("incr", Handler::new_shared_state(
            Arc::new(Mutex::new("Hello Mother".as_bytes().to_vec())),
            Arc::new(|echo| Ok(echo))
        ));
        return map.into();
    }
}
