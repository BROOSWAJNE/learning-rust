fn fib(n: i32) -> i32 {
    if n < 0 { panic!("Oh no!") }
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn fib_test(n: i32) {
    println!("Fibonacci Number #{}: {}", n, fib(n));
}

fn main() {
    for n in 0..15 {
        fib_test(n);
    }
}
