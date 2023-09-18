#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use riscv::register::sstatus::{self, SPP};

fn fib(n: u32) -> u32{
    match n {
        0 => 0,
        1 => 1,
        n => fib(n-1) + fib(n-2)
    }
}

#[no_mangle]
fn main() -> i32 {
    for i in 0..20{
        println!("fib({}) = {}", i, fib(i));
    }
    0
}
