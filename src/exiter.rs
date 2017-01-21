use std::any::Any;
use super::Coworker;
use futures::sync::mpsc::{Sender, Receiver, channel};
use futures::future::{Future, empty};
use futures::stream::Stream;

pub struct Exiter {
    rx: Option<Receiver<()>>,
}

impl Exiter {
    pub fn new(rx: Receiver<()>) -> Exiter {
        Exiter{
            rx: Some(rx),
        }
    }
}

impl Coworker for Exiter {
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

