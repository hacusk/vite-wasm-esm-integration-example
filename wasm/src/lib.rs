#[unsafe(no_mangle)]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let (mut a, mut b) = (0i32, 1i32);
    for _ in 2..=n {
        (a, b) = (b, a + b);
    }
    b
}
