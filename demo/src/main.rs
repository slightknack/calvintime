use std::io::prelude::*;

fn main() {
    let mut file = std::fs::File::open("counter/incr").unwrap();
    file.write_all(b" - And this too!");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    println!("{}", contents);
    println!("The number is: {}", 1 + 3)
}
