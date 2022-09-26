fn fib(n: u32) -> u32 {
    if n == 1 {
        return 0;
    }
    else if n == 2 {
        return 1;
    }

    fib(n-1) + fib(n-2)
}

fn main() {
    let n: u32 = 8;
    let fib_n = fib(n);

    println!("{fib_n}");
}