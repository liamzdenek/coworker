extern crate futures;
extern crate tokio_proto;
extern crate tokio_core;

use futures::stream::Stream;
use futures::Future;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

pub fn main() {
    // Create a new loop
    let mut lp = Core::new().unwrap();

    // Bind to port 4000
    let addr = "0.0.0.0:4000".parse().unwrap();

    // Create the new TCP listener
    let listener = TcpListener::bind(&addr, &lp.handle()).unwrap();

    // Accept each incoming connection
    let srv = listener.incoming().for_each(|socket| {
        // Do something with the socket
        println!("{:#?}", socket);
        Ok(())
    });

    let addr2 = "0.0.0.0:4001".parse().unwrap();

    // Create the new TCP listener
    let listener2 = TcpListener::bind(&addr2, &lp.handle()).unwrap();

    // Accept each incoming connection
    let srv2 = listener2.incoming().for_each(|socket| {
        // Do something with the socket
        println!("2: {:#?}", socket);
        Ok(())
    });

    let combined = srv.select(srv2);

    println!("listening on {:?}", addr);

    lp.run(combined);
}

