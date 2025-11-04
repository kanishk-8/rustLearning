use std::collections::HashMap;

fn main() {
    //vectors
    let mut vec = Vec::new();
    vec.push(34);
    vec.push(45);
    println!("{:?}", vec);

    println!("{:?}", filter_even_numbers(&vec));
    let alter_vec = vec![12, 33, 44, 55, 66, 77, 88];
    println!("{:?}", alter_vec);

    //hashmaps
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("kanishk"), 88);
    println!("{:?}", scores);

    //getting value from hashmap, the default return type is Option<T> : some/none
    let first_score = scores.get("kanishk");
    match first_score {
        Some(score) => println!("Score is {}", score),
        None => println!("No score found"),
    }
}

fn filter_even_numbers(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    // Type annotation can be omitted as Rust can infer the type
    // let mut new_vec: Vec<i32> = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val);
        }
    }
    return new_vec;
}
