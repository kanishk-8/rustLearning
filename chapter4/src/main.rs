// Iterators
use std::vec;

// Function demonstrating the use of iterators in Rust
fn iterators_using_loop() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();
    // Using the iter() method to create an iterator explicitly
    for val in v1_iter {
        println!("value using iter() is: {}", val);
    }
    // Using a for loop to iterate over the vector
    // This automatically creates an iterator and uses it to access each element
    for val in v1 {
        println!("value is: {}", val);
    }
}

fn while_let_iterator() {
    let v3 = vec![100, 200, 300, 400, 500];
    // This needs to be mutable to use next()
    let mut v3_iter = v3.iter();
    while let Some(val) = v3_iter.next() {
        println!("{}", val);
    }
}

fn mutable_iterator() {
    let mut v2 = vec![10, 20, 30, 40, 50];
    // Create a mutable iterator using iter_mut()
    let v2_iter_mut = v2.iter_mut();
    for value in v2_iter_mut {
        *value += 5; // Dereference to modify the actual value
        println!("Modified value using iter_mut(): {}", value);
    }
}
fn into_iterator() {
    let v4 = vec![7, 14, 21, 28, 35];
    // into_iter() takes ownership of the vector and creates an iterator
    let v4_into_iter = v4.into_iter();
    for val in v4_into_iter {
        println!("Value from into_iter(): {}", val);
    }
    // v4 can no longer be used here as its ownership has been moved
    // println!("{:?}", v4);
}

fn iterators_functions() {
    let v = vec![1, 2, 3, 4, 5];
    let v_iter = v.iter();
    // Consuming adaptor: sum()
    let sum: i32 = v_iter.sum();
    // let v2 = v_iter; // This would cause an error as v_iter ownership has been moved to sum()
    println!("Sum of elements using iterator's sum(): {}", sum);

    // Iterator adaptors: map() and filter()
    // Using map() to create a new iterator that adds 1 to each element
    let v2 = vec![10, 20, 30, 40, 50];
    let v2_iter = v2.iter();
    let v2_new = v2_iter.map(|x| x + 1);
    for val in v2_new {
        println!("Value after map(): {}", val);
    }

    // Using filter() to create a new iterator with only even numbers
    let v3 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let v3_iter = v3.iter();
    let v3_filtered = v3_iter.filter(|x| *x % 2 == 0);
    for val in v3_filtered {
        println!("Filtered even value: {}", val);
    }
}

fn odd_then_double() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let v_iter = v.into_iter();
    // let odd_values = v_iter.filter(|x| *x % 2 != 0);
    // let result = odd_values.map(|x| 2 * x);
    // We can chain the iterator adaptors directly
    // .collect() gathers the results into a collection, here a Vec<i32>
    let odd_filtered_doubled: Vec<i32> = v_iter.filter(|x| *x % 2 != 0).map(|x| 2 * x).collect();
    for val in odd_filtered_doubled {
        println!("Odd value doubled: {}", val);
    }
}

fn main() {
    iterators_using_loop();
    mutable_iterator();
    while_let_iterator();
    into_iterator();
    iterators_functions();
    odd_then_double();
}
