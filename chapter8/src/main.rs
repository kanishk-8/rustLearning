use std::{sync::mpsc, thread};

fn main() {
    // Create a channel for communication between threads
    // mpsc stands for multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        // Send the value through the channel
        // this will panic if there is an error sending the message
        // let message = tx.send(val).unwrap();
        let message = tx.send(val);
        match message {
            Ok(_) => println!("Message sent successfully"),
            Err(e) => println!("Error sending message: {}", e),
        }
    });

    // Receive the value from the channel
    let received = rx.recv();
    match received {
        Ok(message) => println!("Received: {}", message),
        Err(e) => println!("Error receiving message: {}", e),
    }
}
