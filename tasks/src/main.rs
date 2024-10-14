use tokio::task;

fn fib (n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n-1) + fib(n-2)
    }
}

#[tokio::main]
async fn main() {
    let a = task::spawn_blocking( || {
        println!("Starting fib(40) computation...");
        let res = fib(40);
        println!("fib(40) = {}", res);
    });
    let b = task::spawn_blocking( || {
        println!("Starting fib(39) computation...");
        let res = fib(39);
        println!("fib(39) = {}", res);
    });
    tokio::join!(a,b).0.unwrap();
}