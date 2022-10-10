use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Function call from C");
}

static HELLO_WORLD: &str = "hello_world";
static mut COUNTER: u32 = 0;

fn main() {
    let mut x: i32 = 5;

    let r1 = &x as *const i32;
    let r2 = &mut x as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
        *r2 = 4;
        println!("r1 is {}", *r1);

        dangerous();

        println!("Calling C's abs func on -3: {}", abs(-3));
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let (v1, v2) = split_at_mut(&mut v, 3);
    println!("v1: {:?}\nv2: {:?}", v1, v2);

    println!("{}", HELLO_WORLD);
    unsafe {
        COUNTER += 2;
        println!("counter: {}", COUNTER);
    }
}

unsafe fn dangerous() {}
unsafe trait Foo { /* Atleast one unsafe method */ }
unsafe impl Foo for i32 {}

fn split_at_mut<T>(v: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = v.len();
    assert!(len >= mid);

    let start = v.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(start, mid),
            slice::from_raw_parts_mut(start.add(mid), len-mid)
        )
    }
}