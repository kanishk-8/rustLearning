fn main() {
    // String
    let greeting = String::from("Hello, world!");
    println!("{}", greeting);

    // String reference
    // This points to a string slice
    let greeting_ref: &str = "Hello, world!";
    println!("{}", greeting_ref);

    // String literal
    // This is stored directly in the binary
    let literal = "This is a string literal.";
    println!("{}", literal);

    let my_string = String::from("Hello there, how are you?");
    let word = first_word(&my_string);
    println!("{}", word);
}

fn first_word(string: &String) -> &str {
    let mut index = 0;
    for i in string.chars() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }
    &string[0..index]
}
