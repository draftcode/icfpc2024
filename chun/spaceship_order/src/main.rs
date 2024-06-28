use std::{cmp, io};

fn surrogate_cost(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    // Chevyshev distance

    return cmp::max((p1.0 - p2.0).abs(), (p1.1 - p2.1).abs());
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    while true {
        let line = stdin.read_line(&mut buffer);
    }
    println!("Hello, world!");
}
