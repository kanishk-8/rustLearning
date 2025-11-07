// Import HashMap from the standard library collections module
use std::collections::HashMap;

fn main() {
    // === VECTORS DEMONSTRATION ===
    // Create a new empty vector that can grow dynamically
    let mut vec = Vec::new();

    // Add elements to the vector using push method
    vec.push(34);
    vec.push(45);

    // Print the vector using debug formatting {:?}
    println!("{:?}", vec);

    // Call our custom function to filter even numbers from the vector
    // Pass a reference to avoid moving ownership
    println!("{:?}", filter_even_numbers(&vec));

    // Alternative way to create a vector with initial values using vec! macro
    let alter_vec = vec![12, 33, 44, 55, 66, 77, 88];
    println!("{:?}", alter_vec);

    // === HASHMAPS DEMONSTRATION ===
    // Create a new HashMap with String keys and i32 values
    // HashMap stores key-value pairs and provides O(1) average lookup time
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Insert key-value pairs into the HashMap
    // String::from() creates an owned String from a string literal
    scores.insert(String::from("kanishk"), 88);
    scores.insert(String::from("manas"), 92);

    // Print the entire HashMap
    println!("{:?}", scores);

    // === SAFE VALUE RETRIEVAL FROM HASHMAP ===
    // get() method returns Option<&V> to handle cases where key might not exist
    // This prevents runtime panics and makes error handling explicit
    let first_score = scores.get("kanishk");

    // Use pattern matching to handle the Option safely
    match first_score {
        Some(score) => println!("Score is {}", score), // Key exists, print the value
        None => println!("No score found"),            // Key doesn't exist
    }
}

// Function that takes a reference to a vector and returns a new vector containing only even numbers
// Using &Vec<i32> as parameter to borrow the vector instead of taking ownership
fn filter_even_numbers(vec: &Vec<i32>) -> Vec<i32> {
    // Create a new empty vector to store the filtered results
    let mut new_vec = Vec::new();
    // Type annotation can be omitted as Rust can infer the type
    // let mut new_vec: Vec<i32> = Vec::new();

    // Iterate through each element in the input vector
    // 'val' is a reference to each element (&i32)
    for val in vec {
        // Check if the number is even using modulo operator
        if val % 2 == 0 {
            // Dereference val with * to get the actual i32 value before pushing
            // This is necessary because val is &i32 but push expects i32
            new_vec.push(*val);
        }
    }

    // Return the new vector containing only even numbers
    // 'return' keyword is optional in Rust - the last expression is automatically returned
    return new_vec;
}
