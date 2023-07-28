use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("***** GUESS THE NUMBER *****");
    
    loop {              
        println!("Enter A Number: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read Line");
        // .expect returns an io::Result (enum of Ok and Err)
        
        let secret_number = rand::thread_rng().gen_range(1, 100);

        println!("You Guessed {}", guess);

        let guess: u32 = guess.trim().parse().
            expect("Please Type A Number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => {
                println!("Correct Guess");
                break;
            },
        }
    }
}
