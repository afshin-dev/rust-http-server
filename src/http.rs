use std::net ;
use std::io::{prelude::* , BufReader} ;
use std::fs ;

pub struct Server {
    host: String , // like 127.0.0.1
    port: String, // :8080

}

impl Server {
    pub fn new(addr: String, port: String) -> Self {
        Self { host: addr, port: port }
    }
    pub fn listen(&self) -> () {
        let connection = net::TcpListener::bind(format!("{}:{}", self.host, self.port));

        // maybe using a unwrap or expect helper must better but for verbosity code written like this 
        let listener = match connection {
            Ok(tcp) => tcp ,
            Err(e) => panic!("{}", e),
        };

        println!("listen on {}", self.port) ;

        for stream in listener.incoming() {
            let  tcp_client = stream.unwrap() ;
            println!("{:?}", tcp_client);
            self.handle_connection(tcp_client);
        }
        // loop {

        //   match listener.accept() {

        //     Ok((mut stream, _)) => {
        //         let mut buffer = [0u8; 1024]; 
                
               
                
              
        //         let res = stream.read( &mut buffer);
                
        //         match res {
        //             Ok(_) => println!("{}",String::from_utf8_lossy(&buffer)) ,
        //             Err(e) => eprintln!("{}", e),
        //         }
                
        //         //stream.write(">".as_bytes()).expect("error");

        //         // println!("{:?}",buffer);
        //     },

        //     Err(e) => {
        //         eprintln!("{}" ,e);
        //     } ,
        //   }
        // }
    }

    fn handle_connection(&self, mut stream: net::TcpStream) -> () {
        let buf_reader = BufReader::new(&mut stream);

        let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

        Self::send(stream, "templates/index.html");

        println!("Request: {:#?}", http_request[0]);
    }

    fn send(mut stream: net::TcpStream, file_name: &str) {
        let status = "HTTP/1.1 200 OK";
        let content = fs::read_to_string(file_name).unwrap();
        let content_lenght = content.len() ;

        let response = format!("{status}\r\nContent-Length: {content_lenght}\r\n\r\n{content}");

        stream.write_all(response.as_bytes()).unwrap();
    }

} 

pub struct Request {
    path : String ,
    query_string : Option<String> ,
    method : Method ,
}   

pub enum Method {
    Get ,
    Post,
    Put,
    Delete,
    Option ,
}