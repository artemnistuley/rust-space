fn main() {
    println!("Hello, rust!");

    another_func(10);
    expression_func();

    let five = get_five();
    println!("{}", five);
}

fn another_func(x: i32) {
    println!("The value of x is: {x}");
}

fn expression_func() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn get_five() -> i32 {
    5
}