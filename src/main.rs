use std::net;
use server::server::Server;


mod server;
mod http;

fn main() {

    // Create a new instance of our HTTP server running on the default address:port
    let mut server = Server::new("127.0.0.1:8080");
    // let it run forever, until terminated with Ctrl-C
    server.run();
}
