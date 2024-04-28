fn main() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("Final x: {x}");

    for x in 1..5 {
        println!("x: {x}");
    }
    for x in 1..=5 {
        println!("x: {x}");
    }
    for elem in [1, 2, 3, 4, 5] {
        println!("elem: {elem}");
    }

    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i >= 100 {
            break;
        }
    }
}
