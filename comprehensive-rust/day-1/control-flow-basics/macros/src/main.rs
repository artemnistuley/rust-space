fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn fizzbuzz(n: u32) {
  for i in 1..=n {
    if i % 15 == 0 {
        println!("FizzBuzz");
    } else if i % 3 == 0 {
        println!("Fizz")
    } else if i % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", i);
    }
  }
}

fn main() {
    let n = 13;
    println!("{n}! = {}", factorial(4));
    
    fizzbuzz(20);

    let formatted = format!("values: {}, {}", 1, 2);
    println!("{}", formatted);

    let arr = [1, 2, 3];
    dbg!(arr);

    // todo!();
    // unreachable!();
}
