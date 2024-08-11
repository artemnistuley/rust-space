use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {HELLO_WORLD}");

    add_to_count(3);
    unsafe {
        println!("COUNTER: {COUNTER}")
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}