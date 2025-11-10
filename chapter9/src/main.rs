// macros

// creating our own macro

macro_rules! myprint {
    ($val:expr) => {
        println!("Value is: {}", $val);
    };
}
fn main() {
    // vec! is a macro that creates a vector
    let v = vec![1, 2, 3];

    let v_iter = v.iter();

    for val in v_iter {
        // println! is a macro that prints to the console
        println!("Got: {}", val);
    }
    myprint!(89);
}
