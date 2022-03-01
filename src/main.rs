use std::io;
use rand::{thread_rng, Rng};
fn main() {
    // the point of the game is to make the user guess a number
    // number is random generated
    // ask user for number
    // check against the randomly generated number
    let mut rng = thread_rng();
    let rand_num: i32 = rng.gen_range(1..101);  

    // while the gueses are not equal
    loop {
        // continue to read from input
        // read the input of the user
        println!("Please input a number");  
        let user_num = reading_integer(); 
        if rand_num < user_num {
            println!("The number is smaller than the number you guessed");
        } 
        else if rand_num > user_num {
            println!("The number is larger than the number you guessed");
        } 
        else {
            break;
        }

        println!("Please guess again");
        println!("Your guess is {}", user_num);
    };
    println!("Congrats you guessed the number {} correctly", rand_num);
    
}

// reading input and return n integer 32
fn reading_integer() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
        // change the string to a number
        // trim function turns the input into a string
        // removes the leading white spaces
    let text = input.trim();
    let mut user_num = 0;

    match text.parse::<i32>() {
        Ok(number) => {
            println!("The input is {}", number);
            // update the number
            user_num = number;
        },
        Err(..) => println!("Input not integer {}", text),
    };
    user_num
}