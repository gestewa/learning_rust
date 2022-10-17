use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess...");
        let mut guess = String::new(); // String is UTF-8 encoded
        let guess_size = io::stdin()
            .read_line(&mut guess)// We could also use std::io::stdin() 
            .expect("failed to read line");
        
        if guess_size>3{
            println!("Your guess is {guess_size} bytes. Don't waste memory")
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("This behavior is highly inappropriate."); continue;},
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => { println!("You win!");break;},
            Ordering::Greater => println!("Too big")
        }
    }
}
