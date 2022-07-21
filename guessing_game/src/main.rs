use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Number guessing game.");
    
    let secret_num = rand::thread_rng().gen_range(1..=100);


    loop {
        //println!("Secret num: {}", secret_num);

        println!("Enter num.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Bad guess!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) =>  num,
            Err(_) => continue,
        };
        
        //.expect("This is guess the NUMBER! ");
        // Shadows previous variable.
        
        println!("Guessed num: {}", secret_num);


        println!("Wow! {guess} is a fantastic guess!");


        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("Bang on!");
                break;
            }
            
            
        }
    }

}
