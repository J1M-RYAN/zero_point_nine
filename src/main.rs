use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} host port request_uri", args[0]);
        eprintln!("eg: {} localhost 8080 /index.html", args[0]);
        return Ok(());
    }

    let host = &args[1];
    let port = &args[2];
    let request_uri = &args[3];

    let address = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(&address).await?;

    let request = format!("GET {}\r\n", request_uri);
    stream.write_all(request.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let mut response = Vec::new();
    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        response.extend_from_slice(&buffer[..n]);
    }
    println!("{}", String::from_utf8_lossy(&response));

    Ok(())
}
