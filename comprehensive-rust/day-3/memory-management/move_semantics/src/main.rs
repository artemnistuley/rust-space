fn say_hello(name: String) {
    println!("name: {name}");
}

fn main() {
    let s1 = String::from("Hello");
    let s2: String = s1;
    println!("s2: {}", s2);
    // println!("s2: {}", s1);

    let name = String::from("Alice");
    say_hello(name);
    // println!("name: {name}");
}
