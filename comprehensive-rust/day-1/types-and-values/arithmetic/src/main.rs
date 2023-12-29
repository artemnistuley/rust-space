fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("result: {}", interproduct(120, 100, 248));
    println!("sum: {}", sum(100, 200));
}
