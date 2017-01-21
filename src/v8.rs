use std::any::Any;
use super::Coworker;
use futures::sync::mpsc::{Sender, Receiver, channel};
use futures::future::{Future, empty};
use futures::stream::Stream;

pub struct V8 {
    rx: Option<Receiver<()>>,
}

impl V8 {
    pub fn new(rx: Receiver<()>) -> V8 {
        V8{
            rx: Some(rx),
        }
    }
}

impl Coworker for V8 {
    fn get_future(&mut self) -> Box<Future<Item=(),Error=()>> {
        box self.rx.take().unwrap()
            .into_future()
            .map(|_| ())
            .map_err(|_| ())
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn notify(&mut self, future: &Box<Coworker>) {
        
    }
}
