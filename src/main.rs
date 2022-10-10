use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    println!("Guess the number!");
    println!("Please input a number. ");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    loop {

        let mut guess = String::new();

        io::stdin()                               // user input 
        .read_line(&mut guess)                    // assigns guess the contents of input
        .expect("Failed to read line!");          // if it cannot, shoot error

        let guess: u32 = match guess.trim().parse() 
        {Ok(num) => num, Err(_) => continue,};                           
        
        
        //.expect("Please type a number instead!"); // throws error if not changeable

        // println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }

        }

    }
}