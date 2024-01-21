/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
// Default to i32
let x = 1;

// Default to f64
let y = 3.5;

// Explicit type
let z: i64 = 434553452345345;

println!("{}, {}, {}", x, y, z);

// Find max size
println!("Max i32: {}, i64: {}, f32: {}, f64: {}", std::i32::MAX, std::i64::MAX,std::f32::MAX,std::f64::MAX);

// Boolean
let is_active = true;
let is_greater = 3 >4;

println!("is active: {}, is greater: {}", is_active, is_greater);

// Char
let a1 = "a";

println!("a1: {a1}");
}