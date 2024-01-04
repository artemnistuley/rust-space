fn main() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
}

/* error 
fn x_axis(x: i32) -> &(i32, i32) {
    let point = (x, 0);
    return &point;
}
*/
