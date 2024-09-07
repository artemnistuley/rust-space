use std::thread;

// fn foo() {
//     let s = String::from("Hello");
//     thread::spawn(|| {
//         println!("Length: {}", s.len());
//     });
// }

fn main() {
    // foo();

    let s = String::from("Hello");
    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Length: {}", s.len());
        });
    });
}
