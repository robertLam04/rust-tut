use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
    
        println!("Input guess");

        // Immutable by default, need mut. String::new() is a function that returns a new string instance.
        // :: - indicates new is an associated function of type String
        // let can only be used in a function
        let mut guess = String::new();

        //Need expect to crash the program when a problem occurs
        io::stdin().read_line(&mut guess).expect("failed to read line!");

        //convert guess to integer
        //Uses shadowing - uses let again to create a new variable
        let guess: u32 = guess.trim().parse().expect("enter a number");

        println!("You guessed: {}", guess);


        // Ordering type is an enum, cmp compares to values, takes in a ref to other value,
        // match expression is made up of arms, arms are patterns to match against
        // code is run if value given to match fits the arms pattern
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("win");
                break;
            },
        }
    }
    


}

