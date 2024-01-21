// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Sam";
    let mut age = 25;
    println!("His name is {}, and he is {} years old", name, age);
    age = 26;
    println!("He is now {}", age);

    // Define constants
    const ID: i32 = 432;

    println!("ID is {}", ID);


    // Assigning multiple vars at a time

    let (new_name, new_age) = ("Victor", 21);

    println!("new person name is {}, and he is {}", new_name, new_age)
}