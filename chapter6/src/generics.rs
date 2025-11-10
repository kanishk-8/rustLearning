fn main() {
    let bigger = largest(10, 20);
    println!("The larger number is {}", bigger);
    let bigger_char = largest('a', 'b');
    println!("The larger character is {}", bigger_char);
}

fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
