// Main entry point of the program
fn main() {
    // Print a greeting message
    println!("hello rust!!");

    // Test the is_even function with the number 4
    println!("is 4 an even number : {}", is_even(4));

    // Calculate and print the 10th number in the fibonacci series
    println!("{}", fibbonacci_series(10));

    // Get and print the length of a string
    println!("{}", get_string_length("nice wow"));

    // Create a new User instance
    let user1 = User {
        id: 1,
        name: String::from("kanishk"),
        age: 21,
    };

    // Call methods on the user instance
    user1.user_intro(); // Display user introduction
    println!("user id is {}", user1.user_id()); // Print user ID
    println!("user is {:?}", user1); // Debug print the entire user struct

    // Demonstrate enum usage with Direction
    let dir = Direction::South;
    // Pattern matching on the Direction enum
    match dir {
        Direction::North => println!("going north"),
        Direction::South => println!("going south"),
        Direction::East => println!("going east"),
        Direction::West => println!("going west"),
    }

    // Create different shapes using enum variants
    let shape1 = Shape::Circle(5.0); // Circle with radius 5.0
    let shape2 = Shape::Rectangle(4.0, 6.0); // Rectangle with length 4.0 and breadth 6.0

    // Calculate and print areas of different shapes
    println!("area of circle is {}", area(shape1));
    println!("area of rectangle is {}", area(shape2));

    // Demonstrate Option<T> usage - find first occurrence of 'a' in a string
    let index = find_first_a(String::from("preet"));
    // Handle the Option result using pattern matching
    match index {
        Some(value) => println!("index is {}", value), // If 'a' is found
        None => println!("a not found"),               // If 'a' is not found
    }

    // Demonstrate Result<T, E> usage
    // Test the is_good function with a positive number
    let result1 = is_good(-5);
    match result1 {
        Ok(value) => println!("Good number: {}", value),
        Err(value) => println!("Not a good number: {}", value),
    }

    // Read a file and handle potential errors
    let read_file_result = std::fs::read_to_string("hello.txt");
    match read_file_result {
        Ok(content) => println!("File content: {}", content), // Print file content if read successfully
        Err(e) => println!("Error reading file: {}", e), // Print error message if reading fails
    }
}

// Function to check if a number is even
fn is_even(num: i32) -> bool {
    // Check if the number is divisible by 2
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

// Recursive function to calculate fibonacci number at given index
fn fibbonacci_series(index: i32) -> i32 {
    // Base cases: first two fibonacci numbers
    let first = 0;
    let second = 1;
    if index == 0 {
        return first;
    }
    if index == 1 {
        return second;
    }

    // Recursive case: sum of previous two fibonacci numbers
    return fibbonacci_series(index - 1) + fibbonacci_series(index - 2);
}

// Function to get the character count of a string (handles Unicode properly)
fn get_string_length(s: &str) -> usize {
    s.chars().count() // Count Unicode characters, not bytes
}

// Derive Debug trait to enable printing the struct with {:?}
#[derive(Debug)]
// User struct to represent a user with id, name, and age
struct User {
    id: i32,
    name: String,
    age: i32,
}

// Implementation block for User struct methods
impl User {
    // Method to introduce the user by printing name and age
    fn user_intro(&self) {
        println!("user name is {} and age of user is {}", self.name, self.age)
    }
    // Getter method to return the user's ID
    fn user_id(&self) -> i32 {
        return self.id;
    }
}

// Derive Debug trait for the enum
#[derive(Debug)]
// Enum representing four cardinal directions
enum Direction {
    North,
    South,
    East,
    West,
}

// Function that takes a Direction and prints movement (unused in main)
fn move_around(dir: Direction) {
    println!("move to {:?}", dir)
}

// Enum with data - represents different geometric shapes
enum Shape {
    Circle(f64),         // Circle variant holds radius
    Rectangle(f64, f64), // Rectangle variant holds length and breadth
}

// Function to calculate area based on shape type
fn area(shape: Shape) -> f64 {
    // Pattern match on shape to calculate appropriate area
    match shape {
        Shape::Circle(r) => 3.14 * r * r, // π * r²
        Shape::Rectangle(l, b) => l * b,  // length * breadth
    }
}

// Function to find the first occurrence of 'a' in a string
// Returns Option<i32> - Some(index) if found, None if not found
fn find_first_a(s: String) -> Option<i32> {
    // Iterate through characters with their indices
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32); // Return index when 'a' is found
        }
    }
    None // Return None if 'a' is not found
}

fn is_good(num: i32) -> Result<i32, i32> {
    if num > 0 { Ok(num) } else { Err(0) }
}
