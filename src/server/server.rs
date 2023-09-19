use std::io::Read;
use std::net::{Shutdown, TcpListener};
use crate::http::request::Request;
use std::thread;

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
    pub fn run(&mut self) {

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
