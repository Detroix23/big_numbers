// src/main.rs
mod modules;

fn main() -> () {
    println!("# Big numbers.");
    println!("## Main tests.");

    let n1: modules::scientific::Scientific = modules::scientific::Scientific::new(1, 1);
    println!("n1 = {}", n1);

    let n2: modules::scientific::Scientific = modules::scientific::Scientific::new(1213, 123);
    println!("n1 = {}", n2);
}