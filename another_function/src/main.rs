// fn main() {
//     println!("Hello, world!");

//     another_function(5);
// }

// fn another_function(x: u8) {
//     println!("Another function. {x}");
// }

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}