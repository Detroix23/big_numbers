// src/main.rs
mod modules;

fn main() -> () {
    println!("# Big numbers.");
    println!("## Main tests.");

    println!("u32::MAX: {}", modules::displays::thousand_separators(std::u32::MAX.to_string(), modules::THOUSANDS));

    println!("### Scientific numbers: base.");

    let n1: modules::scientific::Scientific = modules::scientific::Scientific::new(1, 1);
    println!("n1 = {}", n1);
    let n2: modules::scientific::Scientific = modules::scientific::Scientific::new(1_213, 123);
    println!("n2 = {}", n2);
    let n3: modules::scientific::Scientific = modules::scientific::Scientific::new(633, 123);
    println!("n3 = {}", n3.display_full());
    let n4: modules::scientific::Scientific = modules::scientific::Scientific::new(4_000_000_001, 123);
    println!("n4 = {}", n4);
    let n5: modules::scientific::Scientific = modules::scientific::Scientific::new(4_000_000_000, 123);
    println!("n5 = {}", n5.display_full());


    println!("### Scientific numbers: operations.");

    println!("Add 1: {}", modules::operations::addition(n2, n3));
    println!("Add 2: {}", modules::operations::addition(n4, n5));


}