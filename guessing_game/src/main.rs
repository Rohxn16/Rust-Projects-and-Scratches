use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("WELCOME TO:\n----------------------Guess the number------------------------ ");
    loop{
        println!("Enter your guess here: \n");
        let secret = rand::thread_rng().gen_range(1..=100);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        let guess:u32 = match guess.trim().parse(){
            Ok(num) =>  num,
            Err(_) => {
                print!("Enter a number, Smartass\n\n");
                continue;
            },
        };
        
        // io::stdin().(&mut n).expect("Could no read number");
        println!("The Jackpot number is : {secret}");
        println!("You guesses : {guess}");

        // Comparing the values
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("We have a winner!!!");
                break;
            },
            Ordering::Greater => println!("Thats too big!"),
        }
    }    
}