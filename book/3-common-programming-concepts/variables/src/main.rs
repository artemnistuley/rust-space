const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("My const value is {}", THREE_HOURS_IN_SECONDS);

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("y in the inner scope is: {y}");
    }
    println!("y is {y}")
}
