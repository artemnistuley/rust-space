#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(Direction),
    Teleport { x: u32, y: u32 },
}

fn main() {
    let m: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("{:?}", m);
}
