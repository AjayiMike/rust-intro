// Tuples group together values of different types
// Max 12 elements

pub fn run() {
let student: (&str, i8, &str, f32) = ("Sam", 23, "Computer Science", 4.2);

println!("{} is {}, He graduated from the department of {} with a CGPA of {}", student.0, student.1,student.2,student.3)
}