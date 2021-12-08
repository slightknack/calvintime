fn main() {
    let file = std::fs::read_to_string("counter/incr").unwrap();
    println!("{}", file);
    println!("The number is: {}", 1 + 1)
}
