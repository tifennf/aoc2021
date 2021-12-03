use std::fs;

use day2::{ex1, ex2};

fn main() {
    let inputs = fs::read_to_string("./inputs.txt").unwrap();

    println!("Part 1");

    ex1::run(&inputs);
    println!("\nPart 2");
    ex2::run(&inputs);
}
