use std::env;

pub fn get_args() -> env::Args {
    let mut args = env::args().into_iter();
    args.next(); // skip the first arg - it's the program's name.
    args
}

pub fn get_isizes() -> Vec<isize> {
    let mut data: Vec<isize> = Vec::new();
    let args = get_args();
    for arg in args{
        match arg.parse::<isize>(){  
            Ok(x) => data.push(x),
            Err(_) => println!("could not parse: {arg} to integer isize"),
        }
    }
    data
}



pub fn print_vec_of_ints(data : &Vec<isize>) -> () {
    println!("Parsed integer vals: ");
    for x in data {
        print!("{x}, ")
    }
}

//fn main() {
//   println!("Testing get args. ");
//    let args = get_isizes();
//    print_vec_of_ints(&args);
//}
