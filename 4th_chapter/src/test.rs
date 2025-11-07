use std::collections::HashMap;

fn main() {
    let input_vec = vec![(String::from("kanishk"), 22), (String::from("manas"), 21)];
    println!("input is a vector: {:?}", input_vec);

    let hm = group_values_by_key(input_vec);
    println!("{:?}", hm);
}

fn group_values_by_key(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    hm
}
