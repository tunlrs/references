use tokio::{io::AsyncReadExt, time};
use log::Level;

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n-1) + fib(n-2),
    }
}

async fn sleeper(secs: u64) {
    log::info!("Sleeping");
    time::sleep(time::Duration::from_secs(secs)).await;
    log::info!("Awake!");
}

async fn reader() {
    log::info!("Reading some beeg data");
    let mut f = tokio::fs::File::open("beeg.csv").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read beeg {} bytes", contents.len());

    tokio::task::spawn_blocking(move || {
        log::info!("Computing fib(40)");
        fib(40);
        log::info!("Done computing fib(40)!");
    }).await.unwrap();
}

async fn run() {
    tokio::spawn(async {
        sleeper(2).await
    });
    tokio::join!(
        reader(),
        sleeper(3),
    );
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    
    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();

    println!("Took {:?} seconds", end - start);
}

// fn main() {
//     simple_logger::init_with_level(Level::Info).unwrap();
//     let start = std::time::Instant::now();
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     let future = run();
//     rt.block_on(future);
//     let end = std::time::Instant::now();
//     println!("Took {:?} seconds", end - start);
// }
