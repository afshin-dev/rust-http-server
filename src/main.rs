
const HOST : &str = "127.0.0.1";
const PORT : &str = "7878";

fn main() {

    // println!("'{}'", util::string_slc("127.0.0.1:8080", 0, 10));
    // println!("'{}'", util::string_slc("127.0.0.1:8080", 10, "127.0.0.1:8080".len()));
    let server = http::Server::new(HOST.to_owned(), PORT.to_owned());

    server.listen();
}


pub mod http ;
pub mod util ;