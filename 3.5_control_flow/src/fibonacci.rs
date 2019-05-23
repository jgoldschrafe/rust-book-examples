fn fib_worker(n: u32, two_prev: u32, one_prev: u32) -> u32 {
    match n {
        0 => one_prev,
        _ => fib_worker(n - 1, one_prev, two_prev + one_prev),
    }
}

fn fib(n: u32) -> Result<u32, String> {
    match n {
        0 => Err("invalid index".to_string()),
        1 | 2 => Ok(1),
        _ => Ok(fib_worker(n - 2, 1, 1)),
    }
}

fn main() -> Result<(), String> {
    let n = 10;
    println!("Fibonacci number n is {}", fib(n)?);
    Ok(())
}