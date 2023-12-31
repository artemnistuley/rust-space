fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        return gcd(b, a % b);
    } else {
        return a;
    }
}

fn main() {
    println!("gcd: {}", gcd(143, 52));
}
