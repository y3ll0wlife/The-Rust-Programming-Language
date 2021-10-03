// Convert temperatures between Fahrenheit and Celsius.

use std::io;

fn main() {
    let mut fahren = String::new();

    println!("Calculate fahrenheit to celsius");
    io::stdin()
        .read_line(&mut fahren)
        .expect("failed to read line");

    let fahren: f64 = fahren.trim().parse().expect("failed to conver that line");
    let cels: f64 = (fahren - 32.0) / 1.8;

    println!("{} fahrenheit is {} celsius", fahren, cels)
}
