use std::io::prelude::*;

fn main() {
    let mut file = std::fs::File::with_options().read(true).write(true).open(("counter/incr").unwrap();
    file.write_all(b" - And this too!").unwrap();
    println!("The wrote...");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
    println!("The number is: {}", 1 + 3)
}
