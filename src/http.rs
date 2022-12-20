pub struct Server {
    host: String , // like 127.0.0.1
    port: String, // :8080

}

impl Server {
    pub fn new(addr: String, port: String) -> Self {
        Self { host: addr, port: port }
    }
    pub fn listen(&self) -> () {
        
    }
}