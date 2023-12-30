fn main() {
    let x = 90;
    if x < 20 {
        println!("small");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }

    let x = 10;
    let size = if x < 20 {
        "small"
    } else {
        "large"
    };
    println!("number size: {}", size);
}
