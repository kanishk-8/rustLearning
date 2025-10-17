use std::io;

fn main() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read the line");
    println!("guess the number, enter the number you guessed: {}", guess);
    let num = String::new();
    if guess == num {
        println!("your answer is 32")
    }
    else {
        println!("you lose")
    }

}
