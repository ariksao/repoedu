use std::io;
use std::io::prelude::*;

fn main()
{
    println!("Hjälöö!");
    println!("Wёlkömmen to rust calculator 3000 süper");

    print!("Enter first number: ");
    io::stdout().flush().unwrap(); //outputting a line without \n requires flushing stdout
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to read line");
    let inp = inp.trim().parse().expect("Failed to parse inp");

    print!("Enter second number: ");
    io::stdout().flush().unwrap();
    let mut inp2 = String::new();
    io::stdin().read_line(&mut inp2).expect("Failed to read line");
    let inp2 = inp2.trim().parse().expect("Failed to parse inp2");

	println!("Result: {}",
		add(
			inp2,
			inp
		)
	);
}

fn add(val1:i32, val2:i32) -> i32
{
	return val1 + val2;
}
