extern crate rand;
extern crate mio;

use std::io::*;
//use rand::Rng;
//use std::cmp::Ordering;
use mio::tcp::*;
use mio::*;

mod utils;

const SERVER: mio::Token = mio::Token(0);



fn main() {
    println!("Hedgewars game server, protocol {}", utils::PROTOCOL_VERSION);

    let address = "0.0.0.0:46631".parse().unwrap();
    let server = TcpListener::bind(&address).unwrap();

    let poll = Poll::new().unwrap();
    poll.register(&server, SERVER, Ready::readable(),
               PollOpt::edge()).unwrap();

    let mut events = Events::with_capacity(1024);

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => match server.accept() {
                    Ok((mut client_stream, addr)) => {
                        println!("Connected: {}", addr);
                        client_stream.write_all(
                            format!("CONNECTED\nHedgewars server http://www.hedgewars.org/\n{}\n\n"
                                    , utils::PROTOCOL_VERSION).as_bytes()
                        );
                    },
                    _ => unreachable!()
                },

                _ => unreachable!(),
            }
        }
    }
}