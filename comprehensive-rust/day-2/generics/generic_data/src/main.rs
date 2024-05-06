#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }

    fn set_x(&mut self, x: T) {
        self.x = x;
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let int_and_float = Point2 { x: 2, y: 3.1 };
    println!("{integer:?} and {float:?}");
    println!("coords: {:?}", integer.coords());
    println!("int and float: {:?}", int_and_float);
}
