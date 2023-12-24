fn main() {
    // ownership();
    // ownership2();
    ownership3();
}

fn ownership() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // error
}

fn ownership2() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1);
}

fn ownership3() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // no error
}
