use tokio::time;

async fn count_to(count: i32) {
    for i in 0..count {
        println!("Count in taks {i}!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

#[tokio::main]
async fn main() {
    let task = tokio::spawn(count_to(10));
    
    // count_to(10).await;

    for i in 0..5 {
        println!("Main task: {i}");
        time::sleep(time::Duration::from_millis(5)).await;
    }

    task.await;
}
