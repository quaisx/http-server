use std::io::Read;
use std::net::{Shutdown, TcpListener};
use crate::http::request::Request;
use std::thread;
use crate::http::handler::Handler;
/// HTTP Server
///     - address: IP address:port to bind to
pub struct Server {
    address: String,
}
/// HTTP Server implementation block
impl Server {
    /// Create a new instance of the HTTP Server
    pub fn new(addr: &str) -> Self{
        dbg!(addr);
        Self {
            address: addr.to_string(),
        }
    }
    /// Run an instance of the HTTP Server
    pub fn run(&mut self, mut handler: impl Handler) {

        dbg!(format!("HTTP Server is bound to {} and is running...", &self.address));

        let listener = TcpListener::bind(&self.address).expect("Failed to bind to the address");

        // test with netstat -k -l 8080
        loop {
            // std::thread::sleep(std::time::Duration::from_millis(500));
            let res = listener.accept();
            let (mut stream, addr) = match res {
                Ok((mut stream, addr)) => {
                    (stream, addr)
                },
                Err(e) => {
                    eprintln!("[x] Connection establishment to {} has failed with error: {}", &self.address, e);
                    continue
                }
            };
            let address = self.address.clone();
            thread::spawn(move || {
                dbg!(format!("[+] new connection {} -> {}", &addr.to_string(), address));
                let mut data_buf: Vec<u8> = Vec::new();
                let read_result = stream.read_to_end(&mut data_buf);
                let data_sz = match read_result {
                    Ok(data_sz) => data_sz,
                    Err(e) => {
                        eprintln!("failed to read from the tcp stream. error: {}", e);
                        stream.shutdown(Shutdown::Both).unwrap_or(());
                        return;
                    },
                };
                // now, that the request data has been read in, construct an actual request instance
                // from the byte slice
                let req_result = Request::try_from(&data_buf[..data_sz]);
            });
        }
    }
}


/*
pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
 */