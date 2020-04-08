#![allow(dead_code)] // TODO: remove this at some point
use std::collections::HashMap;

// Lambda prototype: (EventName: String) -> ()
type EventCallback<'a> = Box<dyn FnMut(String) -> () + 'a>;

pub struct EventEngine {
    event: HashMap<String, Vec<EventCallback<'static>>>,
}

// TODO: Add some logs
impl EventEngine {
    pub fn new() -> EventEngine {
        return EventEngine {
            event: HashMap::new(),
        };
    }

    pub fn on<F: 'static>(&mut self, id: String, cb: F)
    where
        F: FnMut(String),
    {
        let f = Box::new(cb);

        match self.event.get_mut(&id) {
            Some(events) => events.push(f),
            None => {
                let events: Vec<EventCallback<'static>> = vec![f];

                self.event.insert(id, events);
            }
        };
    }

    pub fn on_raw<F: 'static>(&mut self, id: &str, cb: F)
    where
        F: FnMut(String),
    {
        self.on(id.to_string(), cb);
    }

    pub fn unon(&mut self, id: String) {
        self.event.remove(&id);
    }

    pub fn unon_raw(&mut self, id: &str) {
        self.unon(id.to_string());
    }

    pub fn is_set(&mut self, id: String) -> bool {
        self.event.contains_key(&id)
    }

    pub fn is_set_raw(&mut self, id: &str) -> bool {
        self.is_set(id.to_string())
    }

    pub fn emit(&mut self, id: String) {
        match self.event.get_mut(&id) {
            Some(callbacks) => {
                for cb in callbacks {
                    cb(id.clone()); // TODO: Clone is bad
                }
            }
            None => println!("Event not set"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_use() {
        let mut x1 = EventEngine::new();
        let t = true;
        let cb = move |s: String| {
            assert!(t);
            assert_eq!(s, "test");
        };

        x1.on_raw("test", cb);
        x1.emit("test".to_string());
    }

    #[test]
    fn multiple_on() {
        let mut x1 = EventEngine::new();
        let t = true;
        let cb1 = move |s: String| {
            assert!(t);
            assert_eq!(s, "test");
        };
        let cb2 = move |s: String| {
            assert!(t);
            assert_eq!(s, "test");
        };

        x1.on_raw("test", cb1);
        x1.on_raw("test", cb2);
        x1.emit("test".to_string());
    }

    #[test]
    fn unon() {
        let mut x1 = EventEngine::new();
        let cb = move |s: String| {
            assert_eq!(s, "test");
        };

        x1.on_raw("test", cb);
        assert!(x1.is_set_raw("test") == true);
        x1.unon_raw("test");
        assert!(x1.is_set_raw("test") == false);
    }
}
