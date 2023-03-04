use std::io; 
use rand::thread_rng; // Brings in the thread_rng method
use rand::Rng; // Brings in the gen_range method
use std::cmp::Ordering; 

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..=100); 
    // let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret nunber is: {secret_number}"); 

    println!("Please input your guess."); 

    let mut guess: String = String::new(); 


    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line"); 

    // Reusing variable names is called shadowing, which is useful for type conversion 
    // Note that how parse also returns a Result enum which allows us to use the expect method
    // let guess: Result<u32> = guess.trim().parse().expect("Please input a number"); 
    let guess = match guess.trim().parse::<u32>(){
        Ok(num) => num,
        Err(_) => {
            println!("Please input a number"); 
            continue; 
        }
    }; 

    println!("You guessed: {guess}"); 

    // Here comp gives out a few cases
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You Win!")
    }
}
