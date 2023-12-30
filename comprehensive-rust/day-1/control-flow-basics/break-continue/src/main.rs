fn main() {
    let (mut a, mut b) = (100, 52);
    let result = loop {
        if a == b {
            break a;
        }
        if a < b {
            b -= a;
        } else {
            a -= b;
        }
    };
    println!("Result: {result}");

    'outer: for x in 1..5 {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
}
