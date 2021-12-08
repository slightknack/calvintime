fn main() {
    let file = std::fs::read("counter/inc").unwrap();
    println!("{}", file);
    println!("The number is: {}", 1 + 1)
}
