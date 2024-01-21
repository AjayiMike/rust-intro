// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    let cgpa = 3.9;
    let was_referred = false;
    let has_impressive_portfolio = true;

    // if/else
    if cgpa >= 4.0 || was_referred {
        println!("Congratulations, you're hired!")
    }else if has_impressive_portfolio {
        println!("You are really good, but luck wasn't on your side today")
    } else {
        println!("Sorry, we moved forward with a better candidate.")
    }

    // shorthand if

    let got_the_job = if cgpa > 4.0 || was_referred {true} else {false};

    println!("Got the job: {}", got_the_job);
}