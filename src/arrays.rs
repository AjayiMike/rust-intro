// Arrays - Fixed list where elements are the same data types

use std::mem;
pub fn run() {
let mut number_array: [i32; 10] = [0,1,2,3,4,5,6,7,8,9];

println!("{:?}", number_array);

// Get single value
println!("At position 0 and 3: {}, {}", number_array[0], number_array[3]);


// Re-assigning value
number_array[3] = 30;

println!("At position 3: {}", number_array[3]);

// Get length
println!("Length: {}", number_array.len());

// Bytes occupied
println!("Array occupies {} bytes", mem::size_of_val(&number_array));

// Get Slice
let slice: &[i32] = &number_array[1..3];
println!("Slice: {:?}", slice);
}