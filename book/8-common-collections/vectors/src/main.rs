fn main() {
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let mut vec = Vec::new();
    vec.push(5);
    vec.push(6);
    vec.push(7);

    // println!("{:?}", vec);

    let vec2 = vec![1, 2, 3, 4, 5];
    let  third: &i32 = &vec2[2];
    // println!("The third elem is {third}");

    let third: Option<&i32> = vec2.get(2);
    match third {
        Some(third) => println!("The third elem is {third}"),
        None => println!("No third elem."),
    }

    let vec3 = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &vec3[100]; // panic
    let does_not_exist = vec3.get(100);

    let vec4 = vec![100, 32, 57];
    for i in &vec4 {
        println!("{i}");
    }

    let mut vec5 = vec![100, 32, 57];
    for i in &mut vec5 {
        *i += 50;
    }
    // println!("{:?}", vec5);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("hello")),
    ];
    println!("{:?}", row);

    {
        let v = vec![1, 2, 3, 4];
    } // <- v goes out of scope and is freed here
}
