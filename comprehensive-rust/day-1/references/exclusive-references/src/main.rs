fn main() {
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    // let mut x_coord = &point.0;
    *x_coord = 20;
    println!("point: {point:?}");
}
