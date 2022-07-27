// Just trying to figure out how to get lifetimes and references to work
// before reading about them properly later on in the rust book after
// the chapter 8 example I'm currently stuck on.

use std::collections::HashMap;



fn main() {
    let mut hm: HashMap<String, Vec<String>> = HashMap::new();
    let mut hmi: HashMap<isize, Vec<&str>> = HashMap::new();
    
    f("key_1".to_string(), & mut hm);

    let i: isize = 5678;
    g(123, & mut hmi);
    g(i, & mut hmi);

    println!("{:?}", hm);
    println!("{:?}", hmi);
}

fn f(key : String, hm : &mut HashMap<String, Vec<String>>) -> () {

    hm.insert(key, Vec::new());


    let const_key: &'static str = "const_key";
    hm.insert(const_key.to_string(), Vec::new());

    let a_key: & str = "a_key";
    hm.insert(a_key.to_string(), Vec::new());


    let new_string: String = String::from(const_key) + " choo choo";
    //let new_key = new_string.as_str();
    hm.insert(new_string, Vec::new());

}

fn g(key : isize, hm : &mut HashMap<isize, Vec<&str>>) -> () {

    hm.insert(key, Vec::new());


    // let const_key: &'static str = "const_key";
    // hm.insert(const_key, Vec::new());

    // let a_key: &'a str = "a_key";
    // hm.insert(a_key, Vec::new());


    // let new_string: String = String::from(const_key) + "choo choo";
    // //let new_key = new_string.as_str();
    // //db.insert(new_string, Vec::new());

}

