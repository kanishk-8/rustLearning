// Define a trait called Summary that provides a common interface
// Traits are similar to interfaces in other languages - they define shared behavior
trait Summary {
    // Default implementation of summarize method
    // Any type implementing this trait can use this default or override it
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Define a Display trait that requires implementing types to provide a display method
// Unlike Summary, this trait has no default implementation
trait Display {
    fn display(&self) -> String;
}
// Define a NewsArticle struct to represent news articles
// This struct holds information about a news article
struct NewsArticle {
    author: String,   // Author of the news article
    headline: String, // Main headline/title
    location: String, // Location where the news occurred
}

// Implement the Summary trait for NewsArticle
// This provides a custom summarize method for news articles
impl Summary for NewsArticle {
    // Override the default summarize method with a custom implementation
    // Returns a formatted string with author, headline, and location
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.author, self.headline, self.location)
    }
}

// Implement the Display trait for NewsArticle
// This provides a custom display method for news articles
impl Display for NewsArticle {
    fn display(&self) -> String {
        String::from("hi from display trait")
    }
}
// Define a Tweet struct to represent social media tweets
// This struct holds information about a tweet
struct Tweet {
    username: String, // Username of the person who tweeted
    content: String,  // The actual tweet content/message
    reply: bool,      // Whether this is a reply to another tweet
    retweet: bool,    // Whether this is a retweet of another tweet
}

// Implement the Summary trait for Tweet
// This provides a custom summarize method for tweets
impl Summary for Tweet {
    // Override the default summarize method with a custom implementation
    // Returns a formatted string with username and tweet content
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Function that accepts any type that implements the Summary trait
// This demonstrates trait bounds using 'impl Summary' syntax
fn notify(item: impl Summary) {
    // Call the summarize method on the item and print the result
    println!("{}", item.summarize());
}

// Alternative function definition using generic type parameters and trait bounds
// This achieves the same as the notify function above
// Here, T is a generic type that must implement the Summary trait
// to be used with this function
fn alt_notify<T: Summary>(item: T) {
    // Call the summarize method on the item and print the result
    println!("{}", item.summarize());
}

// Function that accepts any type implementing both Summary and Display traits
// This demonstrates multiple trait bounds using the '+' syntax
// The type T must implement both traits to be accepted by this function
fn multi_notify<T: Summary + Display>(item: T) {
    println!("{}", item.summarize());
}

// Main function - entry point of the program
fn main() {
    // Create a NewsArticle instance with sample data
    let news1 = NewsArticle {
        author: String::from("Kanishk"),
        headline: String::from("Learning Rust is fun!"),
        location: String::from("India"),
    };
    // Create another NewsArticle instance to demonstrate multi_notify function
    let news2 = NewsArticle {
        author: String::from("manas"),
        headline: String::from("getting choked at josh"),
        location: String::from("josh ka adda"),
    };

    // Call the summarize method directly on the news article and print it
    println!("News Article Summary: {}", news1.summarize());

    // Create a Tweet instance with sample data
    let tweet1 = Tweet {
        username: String::from("kanishk123"),
        content: String::from("Just posted my first Rust program!"),
        reply: false,   // This is not a reply
        retweet: false, // This is not a retweet
    };

    // Call the summarize method directly on the tweet and print it
    println!("Tweet Summary: {}", tweet1.summarize());

    // Demonstrate different functions with trait bounds
    notify(news1); // Uses impl Summary syntax
    alt_notify(tweet1); // Uses generic type parameter with trait bound
    multi_notify(news2); // Uses multiple trait bounds (Summary + Display)
}
