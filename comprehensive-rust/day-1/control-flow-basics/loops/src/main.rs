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

    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i >= 100 {
            break;
        }
    }
}
