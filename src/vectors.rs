// Vectors - resizable arrays

use std::mem;
pub fn run() {
let mut number_vector: Vec<i32> = vec![0,1,2,3,4,5,6,7,8,9];

println!("{:?}", number_vector);

// Get single value
println!("At position 0 and 3: {}, {}", number_vector[0], number_vector[3]);


// Re-assigning value
number_vector[3] = 30;

println!("At position 3: {}", number_vector[3]);

// Push to vector
number_vector.push(100);
println!("number_vector after push operation: {:?}", number_vector);

// Get length
println!("Length: {}", number_vector.len());

// Bytes occupied
println!("Vector occupies {} bytes", mem::size_of_val(&number_vector));

// Get Slice
let slice: &[i32] = &number_vector[1..3];
println!("Slice: {:?}", slice);

// Loop through
for x in number_vector.iter() {
    println!("{}", x)
}

// Loop and mutate
for x in number_vector.iter_mut() {
*x *=2;
}

println!("New vector: {:?}", number_vector)
}