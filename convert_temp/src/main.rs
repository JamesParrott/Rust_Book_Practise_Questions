use std::io;

fn main() {
    let mut input = String::new();

    let output: String = loop {
        println!("Enter temperature (degrees C or F):");


        io::stdin()
            .read_line(&mut input)
            .expect("Oopsie!");
        
        let mut input: String = input.trim()
                                    .parse()
                                    .expect("Bad trim");

        let mut unit = input.pop()
                                .expect("Could not pop last char from input");
                            

        let temperature: f32 = input.trim()
                                    .parse()
                                    .expect("Could not parse input to a number. ");

        unit.make_ascii_uppercase();


        if unit == 'C' {
            println!("Converting from Celsius to Fahrenheit.");
            break format!("{} degrees Celsius is {} degrees Fahrenheit", temperature, 32.0 + temperature * (9.0 / 5.0));
        }
        else if unit == 'F' {
            println!("Converting from Fahrenheit to Celsius.");
            break format!("{} degrees Fahrenheit is {} degrees Celsius", temperature, (temperature - 32.0)* (5.0 / 9.0));
        }

    };
    println!("{}", output);

}
