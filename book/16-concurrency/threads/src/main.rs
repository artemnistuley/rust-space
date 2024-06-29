use std::thread;
use std::time::Duration;

fn main() {
    /* Example 1.
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }
    */


    /* Example 2
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    */

    
    /* Example 3 
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }
    */

    
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
