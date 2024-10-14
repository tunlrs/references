use tokio::sync;

struct MyStruct {
    field: i32,
}

#[tokio::main]
async fn main() {
    let lock = std::sync::Arc::new(
        sync::Mutex::new(MyStruct {field: 0})
    );

    let lock_a = lock.clone();
    let lock_b = lock.clone();

    let a = tokio::spawn(async move {
            let mut val = lock_a.lock().await;
            val.field = 1
        });

    let b = tokio::spawn(async move {
        let mut val = lock_b.lock().await;
        val.field = 2
    });

    tokio::join!(a,b).0.unwrap();

    let val = lock.lock().await;

    println!("Value is: {}", val.field);
}
