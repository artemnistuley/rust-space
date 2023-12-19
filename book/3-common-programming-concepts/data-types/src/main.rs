fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let cat: char = '😻';

    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (t1, t2, t3) = tup;
    let one = tup.2;
    println!("{t2}");
    println!("{one}");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    
    let arr2 = [3; 5]; // let a = [3, 3, 3, 3, 3];
    let first = arr2[0];
    println!("{first}");
}
