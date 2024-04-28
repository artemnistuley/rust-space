fn main() {
    print_tuple((10, 20));
    print_tuple_2((10, 20));
}

fn print_tuple(tuple: (i32, i32)) {
    let left = tuple.0;
    let right = tuple.1;
    println!("left: {left}, right: {right}");
}

fn print_tuple_2(tuple: (i32, i32)) {
    let (left, right) = tuple;
    println!("left: {left}, right: {right}");
}
