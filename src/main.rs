use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, kind player!\nWelcome to Rustaceoneer!\nChoose your class!");
    
    let mut class = String::new();

    io::stdin()
	.read_line(&mut class)
	.expect("Failed to read line");

    if let class = "fighter" {
    	println!("Nice choice!  that's the only one implemented, lol");
    };
}

