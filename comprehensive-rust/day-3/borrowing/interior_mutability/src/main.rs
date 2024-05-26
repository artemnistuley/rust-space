use std::cell::RefCell;
use std::cell::Cell;

fn main() {
    // RefCell
    // Note that `cell` is NOT declared as mutable.
    let cell = RefCell::new(5);

    {
        let mut cell_ref = cell.borrow_mut();
        *cell_ref = 123;

        // This triggers an error at runtime.
        // let other = cell.borrow();
        // println!("{}", *other);
    }
    println!("{cell:?}");


    // Cell
    // Note that `cell` is NOT declared as mutable.
    let cell = Cell::new(5);
    cell.set(123);
    println!("{}", cell.get());
}
