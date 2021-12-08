use crate::handler::HandlerMap;

pub trait Plugin: Send + Sync + std::fmt::Debug {
    type State: Default + Sized;

    // Returns the name of this plugin
    fn name() -> &'static str where Self: Sized;

    // Returns this plugin's API
    fn api() -> HandlerMap where Self: Sized;
}

// #[derive(Default, Debug)]
// struct Counter {
//     number: usize
// }
//
// impl Counter {
//     fn reset(&mut self, read: &mut dyn Read, write: &mut dyn Write) {
//         let mut input = String::new();
//         read.read_to_string(&mut input);
//         let to_reset_to: usize = std::str::FromStr::from_str(&input).unwrap_or(0);
//         self.number = to_reset_to;
//     }
//
//     fn incr(&mut self, read: &mut dyn Read, write: &mut dyn Write) {
//         let mut input = String::new();
//         read.read_to_string(&mut input);
//         let to_increase_by: usize = std::str::FromStr::from_str(&input).unwrap_or(0);
//         self.number += to_increase_by;
//         write.write_all(self.number.to_string().as_bytes());
//     }
// }

// impl Plugin for Counter {
//     fn name() -> &'static str { "counter" }
//
//     fn api() -> HandlerMap<Self> {
//         let mut map: HandlerMap<Self> = HashMap::new();
//         map.insert("incr", Box::new(Counter::incr));
//         map.insert("reset", Box::new(Counter::reset));
//         return map;
//     }
// }
