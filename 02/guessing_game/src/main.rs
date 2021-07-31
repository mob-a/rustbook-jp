use  std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    
    let secret_number = rand::thread_rng().gen_range(1,1000001);
    //    println!("The secret number is {}", secret_number);
        
    loop{
        let mut guess = String::new();    
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }    
}
