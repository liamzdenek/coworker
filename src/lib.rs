#![feature(box_syntax)]

extern crate futures;

mod exiter;
mod v8;

use futures::future::{Future, empty};
use futures::IntoFuture;
use futures::stream::Stream;

use std::any::Any;

trait Coworker {
    fn notify(&mut self, &Box<Coworker>);
    fn get_future(&mut self) -> Box<Future<Item=(), Error=()>>;
    fn as_any(&self) -> &Any;
}

fn thread_worker(mut coworkers: Vec<Box<Coworker>>) {
    let acc: Box<Future<Item=(), Error=()>> = box empty();

    let mut new_coworkers = vec![];

    for i in 0..coworkers.len() {
        let mut coworker = coworkers.swap_remove(0);
        
        for tcoworker in coworkers.iter_mut() {
            coworker.notify(tcoworker);
            tcoworker.notify(&coworker);
        }

        new_coworkers.push(coworker);
    }

    let future: Box<Future<Item=(), Error=()>> = new_coworkers.into_iter()
        .map(|mut coworker| coworker.get_future())
        .fold(acc, |acc, future| {
            let combined = acc.select(future);//.and_then(|_| Ok(()))

            return box combined
                .map(|_| ())
                .map_err(|_| ());
        })
    ;

    // will wait on this line until any future resolves, but they won't until they wish to kill the thread
    future.wait().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_thread() {
        use std::thread;
        use futures::Sink;
        use futures::Future;
        use super::exiter::Exiter;
        use super::thread_worker;
        use futures::sync::mpsc::channel;

        let (tx, rx) = channel(1);
        
        let err = tx.send(()).wait();
        err.unwrap();

        // this function should return almost-immediately
        thread_worker(vec![
            box Exiter::new(rx)
        ])
    }
}
