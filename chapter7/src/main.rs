// Lifetimes and Threads in Rust

use std::{thread, time::Duration};
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() { str1 } else { str2 }
}

// Example of struct with lifetime annotation
// The user is valid as long as the string slice it holds is valid
struct User<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // let ans;
    // let str1 = String::from("hello");
    // {
    //     let str2 = String::from("world");
    //     ans = longest(&str1, &str2);
    // }
    // println!("The longest string is: {}", ans);
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi from number:{} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // Wait for the spawned thread to finish
    // Wait for the spawned thread to finish
    // let handle = thread.spawn(f)
    // handle.join().unwrap();
    for i in 1..5 {
        println!("hi from number:{} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Example of moving ownership into a thread
    let x = 5;
    // Move ownership of x into the thread
    let handle = thread::spawn(move || {
        println!("The value of x is: {}", x);
    });
    handle.join().unwrap();
}
