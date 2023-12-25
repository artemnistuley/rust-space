fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("{}", max),
        _ => (),
    }

    let config_max2 = Some(3u8);
    if let Some(max) = config_max {
        println!("{}", max);
    }

    let my_value:Option<u32> = None;
    if let Some(val) = my_value {
        println!("{}", val);
    } else {
        println!("None!");
    }
}
