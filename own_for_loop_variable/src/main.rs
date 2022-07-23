// Try to borrow a moveable for loop variable.  
// Result: for loop variables not implementing Copy such as Strings can still be 
// moved, and cannot be borrowed thereafter.
// Hence it is intentional this will not compile - it is a consequence of the 
// above test result.

fn main() {
    let words = [String::from("Hey"), String::from("You"), String::from("We")];
    for s in words{
        let s2 = s;
        println!("Word == {s}!");
        println!("s2 == {s2}");
    }
}

// Expected:
// error[E0382]: borrow of moved value: `s`
//   --> own_for_loop_variable\src\main.rs:11:28
//    |
// 9  |     for s in words{
//    |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
// 10 |         let s2 = s;
//    |                  - value moved here
// 11 |         println!("Word == {s}!");
//    |                            ^ value borrowed here after move
