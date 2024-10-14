use tokio::time;

async fn do_something() -> i32 {
    time::sleep(time::Duration::from_secs(1)).await;
    5
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_do_something() {
        let res = do_something().await;
        assert_eq!(res, 5);
    }

    #[tokio::test]
    async fn test_do_something_again() {
        let res = do_something().await;
        assert_eq!(res, 5);
    }
}
