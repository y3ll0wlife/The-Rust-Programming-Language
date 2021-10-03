// Generate the nth Fibonacci number.

fn fib(n: i64) -> u128 {
    let mut curr = 0;
    let mut n1 = 0;
    let mut n2 = 1;

    for _ in 0..(n - 1) {
        curr = n1 + n2;
        n1 = n2;
        n2 = curr;
    }

    curr
}

fn main() {
    for x in 0..187 {
        println!("fib of {} is {} ", x, fib(x));
    }
}
