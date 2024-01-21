pub fn run() {
    great("Good evening", "Sam");
    let result = add(43, 23);
    println!("Result: {}", result);

    // Clossure
    let num = 16;
    let clossure_add = |n1: i32, n2: i32| n1 + n2 + num;

    println!("Clossure: {}", clossure_add(3,1))
}

fn great(greeting: &str, name: &str) {
    println!("{}, {}, How do you do?", greeting, name)
}

fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}