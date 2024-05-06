fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

fn show<T>(a: T)
where
    T: std::fmt::Display + std::fmt::Debug
{
    println!("{a:?}");
}

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");
    show(String::from("foo"));
}
