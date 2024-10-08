use futures::future::join_all;
use std::time::Instant;
use tokio::time;

// https://google.github.io/comprehensive-rust/concurrency/async-pitfalls/blocking-executor.html

async fn sleep_ms(start: &Instant, id: u64, duration_ms: u64) {
    // std::thread::sleep(std::time::Duration::from_millis(duration_ms));
    time::sleep(time::Duration::from_millis(duration_ms)).await;
    println!(
        "future {id} slept for {duration_ms}ms, finished after {}ms",
        start.elapsed().as_millis()
    );
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let start = Instant::now();
    let sleep_futures = (1..=10).map(|t| sleep_ms(&start, t, t * 10));
    join_all(sleep_futures).await;
}