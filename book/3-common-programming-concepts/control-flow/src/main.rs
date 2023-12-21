fn main() {
    // check_condition();
    // check_condition2();
    // check_condition3();
    // check_condition4();
    // check_loop();
    // check_loop2();
    // check_loop3();
    // check_loop4();
    // check_loop5();
    check_loop6();
}

fn check_condition() {
    let num = 7;
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn check_condition2() {
    let num = 3;
    if num != 0 {
        println!("number was something other than zero");
    }
}

fn check_condition3() {
    let num = 6;

    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn check_condition4() {
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("The value of number is: {num}");
}

fn check_loop() {
    loop {
        println!("again");
    }
}

fn check_loop2() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn check_loop3() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn check_loop4() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn check_loop5() {
    let a = [10, 20, 30, 40, 50];
    for elem in a {
        println!("{elem}");
    }
}

fn check_loop6() {
    for num in (1..10).rev() {
        println!("{num}");
    }
    println!("LIFTOFF!!!");
}
