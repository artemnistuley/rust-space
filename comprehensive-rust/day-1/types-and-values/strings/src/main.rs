fn main() {
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("final sentence: {}", sentence);
    println!("{:?}", &sentence[0..5]);
    println!("{:?}", &sentence[11..15]);

    let mut username = String::new();
    username.push_str("Joe");
    username.push_str(" Doe");
    println!("username: {username}");
}
