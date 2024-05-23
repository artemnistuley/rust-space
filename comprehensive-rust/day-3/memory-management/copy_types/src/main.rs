#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn main() {
    let x = 42;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");

    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}

