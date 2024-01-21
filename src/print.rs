pub fn run() {
    // print to console
    println!("Hello, from the print.rs file");

    // Basic formatting
    println!("She is a {}, and more. Maybe {}", 10, 11);

    // Positonal arguments
    println!("{0} is a {1} guy, and {0} likes to {2}", "Sam", "great", "gym");

    // Named Arguments
    println!("{name} likes to play {sport}", name = "Sam", sport = "football");

    // Placeholder traits
    println!("Binary: {:b}, Octal: {:o}, Hex: {:x}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "A boy"));

    // Basic math
    println!("5 + 3 = {}", 5 +3)
}