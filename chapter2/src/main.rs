use chrono::Utc;
fn main() {
    let now = Utc::now();
    let a: i8 = 43;
    println!("{} and type is ", a);
    println!("{}", now);
    // let mut a: &str = "kanishk";
    // let b = a;
    // println!("{}", b);
    // a = "kanishk gupta";
    // println!("{}", a);
    // a = "hello";
    // println!("{}", a);

    // Ownership example - Uncommenting the below code will cause a compile-time error

    // let a = String::from("kanishk");
    // let b = a;
    // println!("{}", a);

    // To fix the ownership issue, we can use the clone method

    // let a = String::from("kanishk");
    // let b = a.clone();
    // println!("a = {}, b = {}", a, b);

    let mut s1 = String::from("helloworld");
    do_something(&mut s1);
    println!("{}", s1);
}

fn do_something(s2: &mut String) {
    s2.push_str(" kanishk gupta");
    println!("{}", s2);
}
