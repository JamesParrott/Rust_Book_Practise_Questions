use std::io;

fn main() {
    let mut prev: u128 = 0;
    let mut current: u128 = 1;
    let mut tmp: u128;
    let n: i32 = get_num("Calculates n'th Fibonacci number, n >= 2.  Input n:");
    println!("n == {n}");
    for _ in 2..n {
        tmp = current;
        current += prev;
        prev = tmp;
    }
    println!("The {n}'th Fibonacci number is {current}")

}

fn get_num(prompt: &'static str) -> i32 {


    println!("{}", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Oopsie!");
    
    let input: String = input.trim()
                             .parse()
                             .expect("Bad trim");

                        

    let number: i32 = input.trim()
                           .parse()
                           .expect("Could not parse input to a number. ");

    number
}