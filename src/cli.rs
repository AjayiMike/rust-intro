use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  let command = args[1].clone();
  let name = "Sam";
  let state = "happy";

  // println!("Command: {}", command);

  if command == "greet" {
    println!("Good day {}, how are you?", name);
  } else if command == "state" {
    println!("State is {}", state);
  } else {
    println!("Invalid command!!");
  }
}