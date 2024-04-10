#![no_std]
#![no_main]

use nexus_rt::{println, Write};

fn fib(n: u32) -> u32 {
    let mut sum: u32;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }
    sum
    // match n {
    //     0 => 1,
    //     1 => 1,
    //     _ => fib(n - 1) + fib(n - 2),
    // }
}

#[nexus_rt::main]
fn main() {
    for n in 0..10 {
        println!("fib({n}) = {}", fib(n));
    }
}
