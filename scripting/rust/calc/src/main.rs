use std::io;
use std::io::prelude::*;

const TOTAL_INPUTS: isize = 2;

fn main()
{
    println!("Hjälöö!");
    println!("Wёlkömmen to rust calculator 3000 süper");

    match TOTAL_INPUTS {
        0 => {panic!("What are you even trying to achieve?");}
        1 => {panic!("TOTAL_INPUTS is lower than 2");}
        _ => ()
    }

    let mut inputs: isize = 0;
    let mut succ_input_counter: isize = 0;

    while succ_input_counter < TOTAL_INPUTS {
        let res: isize = match get_number_from_input("Enter a valid number: ") {
            None => {println!("No valid number was provided"); continue;},
            Some(val) => {succ_input_counter += 1; val}   
        };

        inputs += res;
    }

	println!("Result: {}\n", inputs);

}

fn get_number_from_input(text: &str) -> Option<isize> {
    print!("{}", text);

    io::stdout().flush().unwrap(); //outputting a line without \n requires flushing stdout
    let mut inp = String::new();

    match io::stdin().read_line(&mut inp) {
        Ok(_) => {},
        Err(_) => {return None}
    };
    let inp: isize = match inp.trim().parse() {
        Ok(val) => val,
        Err(_) => return None
    };

    Some(inp)
}
