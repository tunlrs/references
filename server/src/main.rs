use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net;
use std::str;

#[tokio::main]
async fn main() {
    let host = "0.0.0.0:8080";
    let srv = net::TcpListener::bind(host).await.unwrap();
    loop {
        let (mut sock, _) = srv.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let n = sock.read(&mut buf).await.unwrap();
            sock.write_all(&buf[0..n]).await.unwrap();
            let data = str::from_utf8(&buf[0..n]).unwrap();
            println!("Echoed {:?}", data);
            sock.shutdown().await.unwrap();
        });
    }
}
