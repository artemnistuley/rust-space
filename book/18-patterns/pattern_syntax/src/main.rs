struct Point {
    x: i32,
    y: i32,
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Message3 {
    Hello { id: i32 },
}

fn main() {
    let x = 4;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }


    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");


    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }


    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }


    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }


    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b} = p;
    println!("{a}, {b}");


    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("{x}, {y}");


    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }


    let msg = Message::Move{ x: 1, y: 2 };
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }


    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }


    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{feet}, {inches}, {x}, {y}");


    foo(3, 4);


    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {setting_value:?}");


    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }


    let _x = 5;
    let y = 10;


    // let s = Some(String::from("Hello!"));
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    // println!("{s:?}");


    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{s:?}");


    let origin = Point2 { x: 0, y: 0, z: 0 };
    match origin {
        Point2 { x, .. } => println!("x is {x}"),
    }


    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }


    let num = Some(5);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }


    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");


    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }


    let msg = Message3::Hello { id: 5 };
    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {id}"),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}
