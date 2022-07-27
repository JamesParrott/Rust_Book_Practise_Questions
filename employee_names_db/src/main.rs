use std::io;
use get_args::get_args;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

mod commands {
    pub const ADD_NAME: &str = "add";
    pub const NEW_DEPT: &str = "new";
    pub const SHOW_DEPT: &str = "show";
    pub const SHOW_ALL: &str = "show all";
    pub const QUIT: &str = "quit";
    pub const TO: &str = "to";
}


fn main() {
    let mut db: HashMap<String, Vec<String>> = HashMap::new();
 
    let args = get_args();
    for arg in args {
        process_command(arg, & mut db);
    }
    loop{
        println!("Enter Command ({} NAME [{}] DEPT/{} DEPT/{} DEPT/{}/{}): "
                ,commands::ADD_NAME, commands::TO, commands::NEW_DEPT
                ,commands::SHOW_DEPT, commands::SHOW_ALL, commands::QUIT
                );
        let mut command = String::new();
        io::stdin().read_line(&mut command)
                   .expect("Could not read command. ");
        command = command.trim().to_string();
        let quit: bool = process_command(command, & mut db);
        if quit == true {
            break;
        }
        //println!("{:?}",db);
        println!("");
    }
}

fn process_command(command_str : String, db : & mut HashMap<String, Vec<String>>) -> bool {
    let mut command_iter = command_str.trim_start()
                                                   .split_word_bounds();
                                                    // includes separators.

    let first_word_lc = command_iter.next().unwrap().to_lowercase();
    command_iter.next(); // Discard separator after first command word
    let command_words: Vec<& str> = command_iter.collect();
    let command_args_str: String = command_words.concat().to_string();

    if command_str.to_lowercase().starts_with(commands::SHOW_ALL.to_lowercase().as_str()){
        let mut depts: Vec<_> = db.keys().collect();
        depts.sort_unstable();
        for dept in depts {
            show_dept_employees(dept.to_string(), &db);
            println!("");
        } 
    } else {
        //let first_word_lc: String = command_words[0].to_lowercase();
        match first_word_lc.as_str() {
            commands::SHOW_DEPT => {
                show_dept_employees(command_args_str, &db);
            }
            commands::NEW_DEPT => {
                db.insert(command_args_str, Vec::new());
            }
            commands::ADD_NAME => {
                let to = commands::TO.to_lowercase();
                if command_args_str.to_lowercase().contains(&to) {
                    let mut add_name_words_iter = command_words.rsplit(|word| &word.to_lowercase() == &to);
                    let dept = add_name_words_iter.next().unwrap().concat().trim().to_string();
                    let employee = add_name_words_iter.next().unwrap().concat().trim().to_string();
                    // println!("dept == {}", dept);
                    // println!("employee == {}", employee);
                    let employees = db.entry(dept).or_insert(Vec::new());
                    employees.push(employee);
                }
            }
            commands::QUIT => return true,
            _ => (),


        }
    }
    false
}


fn show_dept_employees(dept : String, db : &HashMap<String, Vec<String>>) -> (){
    println!("{}: ",dept.to_uppercase());
    if let Some(dept_employees) = db.get(&dept){
        let mut sorted_employees = dept_employees.clone();
        sorted_employees.sort_unstable();
        for employee in sorted_employees{
            println!("{},",employee);
        } 
    }
    else {
    println!("No employees. ");
    }
}