const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

static BANNER: &str = "Welcome!";

fn main() {
    println!("{}, {:?}", DIGEST_SIZE, ZERO);
    println!("{BANNER}");
}
