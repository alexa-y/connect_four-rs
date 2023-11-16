use std::io;
use std::net::TcpListener;
use crate::client::Client;

pub fn listen() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:54321")?;

    println!("Listening on {:?}", listener.local_addr().unwrap());

    let mut _stream = listener.accept()?;

    Client::new(_stream.0, 1).process()
}
