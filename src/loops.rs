pub fn run() {
    let mut count = 0;

    // infinit loop
    loop {
        count +=1;
        println!("Count: {}", count);

        if count == 20 {break;}
    }

    // while loop
    while count < 100 {
        if count % 3 == 0 && count % 5 == 0 {
            println!("FizzBuzz")
        } else if count % 3 == 0 {
            println!("Fizz")
        } else if count % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", count)
        }

        count +=1;
    }

    // for range
    for x in 1..100 {
        if x % 3 == 0 && x % 5 == 0 {
            println!("FizzBuzz")
        } else if x % 3 == 0 {
            println!("Fizz")
        } else if x % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", x)
        }
    }

}
