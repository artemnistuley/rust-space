use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Count in thread: {i}");
            thread::sleep(Duration::from_millis(5));
        }

        10
    });

    let result = handle.join().unwrap();

    println!("Result: {result}");

    for i in 0..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
}
