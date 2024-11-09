use std::net::TcpListener;
use std::io::{Read, Write};

const PORT: u16 = 8080;
const BUFFER_SIZE: usize = 1024;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT))?;
    println!("Server is listening on port {}", PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0u8; BUFFER_SIZE];
                stream.read(&mut buffer)?;
                println!("Received: {}", String::from_utf8_lossy(&buffer));

                let response = "HTTP/1.0 200 OK\r\n
                    Server: webserver-c\r\n
                    Content-type: text/html\r\n\r\n
                    <html>hello, world</html>\r\n";
                stream.write(response.as_bytes())?;
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}

