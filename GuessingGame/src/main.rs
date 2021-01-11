use std::io::stdin as input;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    
    println!("Guess the number [1-100]!");
    
    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        input().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess
                    .trim()
                    .parse() {
                        Ok(num) => num,
                        Err(_) => {
                                println!("Invalid input!");
                                break;
                            },
                    };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                    println!("You win!");
                    break;
            }
        }
    }
}