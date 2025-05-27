use std::io;
use rand::Rng;
fn main() {
    let mut user = "Alice";
    println!("Hello, {}!", user);
    user = "Bob";
    println!("Hello, {}!", user);
    println!("Goodbye, {}!", user);
    integers();
    characters();
    booleans();
    arrays();
    forloopwithme();
    whileloopwithme();
    userprint(user, "kanishk", 1200);
    guessthenumber();
}

fn integers() {
    // signed integer
    let a: i32 = 10;
    // unsigned integer
    let b: u32 = 20;
    // floating point number
    let c: f64 = 30.5;

    println!("Signed integer: {}", a);
    println!("Unsigned integer: {}", b);
    println!("Floating point number: {}", c);
}

fn characters() {
    let c: char = 'A';
    let d: char = 'B';
    let e: char = 'C';

    println!("Character 1: {}", c);
    println!("Character 2: {}", d);
    println!("Character 3: {}", e);
}

fn booleans(){
    let is_true: bool = true;
    let is_false: bool = false;

    println!("Boolean true: {}", is_true);
    println!("Boolean false: {}", is_false);
}

fn userprint(user: &str, sender: &str, time: i32) {
    println!("Hello, {user}! from {sender} at time @ {time} hours");
    // or we can use format! macro
    // println!("Hello, {}! from {} at time @ {} hours", user, sender, time);
    
}

fn arrays() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Accessing elements in the array {:?} is used for debug formatting
    println!("Array elements: {:?}", arr);
}
fn forloopwithme(){
    for i in 1..=10{
        println!("Loop iteration: {}", i);
    }
}

fn whileloopwithme(){
    let mut i = 1;
    while i <= 12  {
        println!("while loop interactions: {} ", i);
        i += 1;
    }
}

fn guessthenumber(){
    println!("Please enter a number: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    // random number generating using the rand crate
    let secret_number = rand::rng().random_range(1..=100); // Generates a random number between 1 and 100
    guess = guess.trim().to_string(); // Trim whitespace and convert to String
    let guess: u32 = match guess.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };
    if guess == secret_number {
        println!("Congratulations! You guessed the number: {}", secret_number);
    } else {
        println!("Sorry, the secret number was: {}", secret_number);
    }
}