fn main() {
    let input = 'X';
    match input {
        'q'                       => println!("Quitting"),
        'a' | 's' | 'w' | 'd'     => println!("Moving around"),
        '0'..='9'                 => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        key if key.is_uppercase() => println!("Uppercase: {key}"),
        _                         => println!("Something else"),
    }
}
