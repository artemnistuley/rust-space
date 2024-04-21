fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // error
    // println!("{}, {}", r1, r2);


    let mut s2 = String::from("hello");
    {
        let r3 = &mut s2;
    }
    let r4 = &mut s2;


    let mut s3 = String::from("hello");
    let r5 = &s3;
    let r6 = &s3;
    // let r7 = &mut s3; // error
    // println!("{}, {}, and {}", r5, r6, r7);


    let mut s4 = String::from("hello");
    let r8 = &s4;
    let r9 = &s4;
    println!("{} and {}", r8, r9);
    let r10 = &mut s4; // no error
    println!("{}", r10);


    // let reference_to_nothing = dangle(); error
    let no_dangle_var = no_dangle();
    println!("{}", no_dangle_var);
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
