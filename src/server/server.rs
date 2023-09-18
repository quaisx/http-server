

use std::net::TcpListener;


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
            let (stream, addr) = match res {
                Ok((stream, addr)) => {
                    (stream, addr)
                },
                Err(e) => {
                    eprintln!("[x] Connection establishment to {} has failed with error: {}", &self.address, e);
                    continue
                }
            };

            dbg!(format!("[+] new connection {} -> {}", &addr.to_string(), &self.address));
        }
    }
}
