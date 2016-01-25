extern crate rustc_serialize;
extern crate hyper;
extern crate mio;
mod irc;
mod matrix;
mod bridge;
use mio::{EventLoop,Handler,Token,EventSet,PollOpt};
use std::thread;
use bridge::Bridge;
use std::env;

struct IrcHandler {
    server: irc::streams::Server,
    url: String
}

impl Handler for IrcHandler {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<IrcHandler>, token: Token, _: EventSet) {
        match token {
            SERVER => {
                match self.server.accept() {
                    Some(client) => {
                        let mut bridge = Bridge::new(client, self.url.trim());
                        thread::spawn(move||{
                            bridge.run()
                        });
                    },
                    None => ()
                }
            },
            _ => unreachable!()
        }
    }
}

const SERVER: Token = Token(0);

fn main() {
    let addr = "127.0.0.1:8001".parse().unwrap();
    let args: Vec<_> = env::args().collect();
    let url =  env::args().nth(1).unwrap();
    let server = irc::streams::Server::new(&addr);
    println!("Listening on 127.0.0.1:8001");
    let mut events = EventLoop::new().unwrap();
    events.register(server.listener(), SERVER, EventSet::all(), PollOpt::edge()).unwrap();
    events.run(&mut IrcHandler{
        server: server,
        url: url
    }).unwrap();
}
