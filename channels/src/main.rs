
#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
        }
    });
    while let Some(value) = rx.recv().await {
        println!("Got = {}", value)
    }
}
