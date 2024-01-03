fn main() {
    describe_point((-1, -1));

    my_func();

    let arr = [1, 2, 3];
    match arr {
        [1, _, _] => println!("First is 1"),
        _ => ()
    }
}

fn describe_point(point: (i32, i32)) {
    match point {
        (0, _) => println!("on Y axis"),
        (_, 0) => println!("on X axis"),
        (x, _) if x > 0 => println!("left of Y axis"),
        (_, y) if y < 0 => println!("below X axis"),
        _ => println!("first quadrant"),
    }
}

fn my_func() {
    let triple = [1, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..] => println!("First is 1 and the rest were ignored"),
        _ => println!("All elements were ignored"),
    }
}
