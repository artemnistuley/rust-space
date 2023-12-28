fn main() {
    let mut s = String::new();

    let data = "initial";
    let s = data.to_string();

    let s = String::from("initial");

    let hello = String::from("Hello");
    let hello = String::from("Olá");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved
    // println!("{s1}"); // error

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let s1 = String::from("hello");
    // let h = s1[0]; // error

    let hello = "Здравствуйте";
    let s = &hello[0..4];

    for c in "Зд".chars() {
        println!("{c}");
    }
    for c in "Зд".bytes() {
        println!("{c}");
    }
}
