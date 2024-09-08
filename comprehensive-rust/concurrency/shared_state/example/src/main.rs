use std::thread;
use std::sync::{Arc, Mutex};

// fn main() {
//     let v = vec![10, 20, 30];
//     let handle = thread::spawn(|| {
//         v.push(10);
//     });
//     v.push(1000);

//     handle.join().unwrap();
//     println!("v: {v:?}");
// }

fn main() {
    let v = Arc::new(Mutex::new(vec![10, 20, 30]));

    let v2 = Arc::clone(&v);
    let handle = thread::spawn(move || {
        let mut v2 = v2.lock().unwrap();
        v2.push(10);
    });

    {
        let mut v = v.lock().unwrap();
        v.push(1000);
    }

    handle.join().unwrap();

    println!("v: {v:?}");
}
