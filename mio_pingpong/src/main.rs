extern crate mio;

use mio::tcp::*;

const SERVER: mio::Token = mio::Token(0);

struct Pong {
    server: TcpListener,
    connections: Slab<Connection>,
}

struct Connection {
    socket: TcpStream,
    token: mio::Token,
    state: State,
}

enum State {
    Reading(Vec<u8>),
    Writing(Take<Cursor<Vec<u8>>>),
}

impl mio::Handler for Pong {
    type Timeout = ();
    type Message = ();

    fn new(server: TcpListener) -> Pong {
        // Create a Slab with capacity 1024. Skip Token(0).
        let slab = Slab::new_starting_at(mio::Token(1), 1024);

        Pong {
            server: server,
            connections: slab,
        }
    }

    fn ready(&mut self, event_loop: &mut mio::EventLoop<Pong>,
             token: mio::Token, events: mio::EventSet) {
        match token {
            SERVER => {
                // Only receive readable events
                assert!(events.is_readable());

                println!("the server socket is ready to accept a connection");
                match self.server.accept() {
                    Ok(Some(socket)) => {
                        println!("accepted a socket");
                        let token = self.connections
                            .insert_with(|token| Connection::new(socket, token))
                            .unwrap();

                        event_loop.register_opt(
                            &self.connections[token].socket,
                            token,
                            mio::EventSet::readable(),
                            mio::PollOpt::edge() | mio::PollOpt::oneshot()).unwrap();
                    }
                    Ok(None) => {
                        println!("the server socket wasn't actually ready");
                    }
                    Err(e) => {
                        println!("listener.accept() errored: {}", e);
                        event_loop.shutdown();
                    }
                }
            }
            _ => {
                self.connections[token].ready(event_loop, events);
            }
        }
    }
}

fn main() {
    let address = "0.0.0.0:6567".parse().unwrap();
    let server = TcpListener::bind(&address).unwrap();

    let mut event_loop = mio::EventLoop::new().unwrap();
    event_loop.register(&server, SERVER);

    println!("running pingpong server");
    let mut pong = Pong::new(server);
    event_loop.run(&mut pong);
}
