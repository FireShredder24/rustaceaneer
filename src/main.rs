use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io;

fn main() {
    println!("Hello, kind player!\nWelcome to Rustaceoneer!\nChoose your class!");

    fn read_pop() -> std::string::String {
	let mut s = String::new();
	io::stdin()
	    .read_line(&mut s)
	    .expect("Failed to read line");
   
	s.pop();

	return s;
    }

    if read_pop() == "fighter" {
    	println!("Nice choice!  (that's the only one implemented, lol)\n");
	let con = 12;
	let str = 15;
	let dex = 14;
	let wis = 9;
	let intel = 9;
	let cha = 10;
    }

    println!("Watch yourself, there are mean things out there.\n");

    fn ask(q: String) -> bool {
	println!("{}", q);
	let mut a = String::new();
	if read_pop() == "y" {
	    return true;
	} else {
	    return false;
	}
    }

    println!("You're in the middle of a small market, surrounded by the houses of the people who are too rich to frequent the vendors at their doorstep.\nThere are 4 vendors there, 2 of them selling clothes, 2 selling food.  They are unfamiliar to you, but by their garments you can tell\nthey are not rich.");

    if ask("Do you want to kill the guy?".to_string()) {
	println!("cyka blyat!");
    }

    let path = Path::new("testsave.txt");
    let display = path.display();

    
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
	Ok(file) => file,
    };

    let text = read_pop();

    match file.write_all(text.as_bytes()) {
	Err(why) => panic!("couldn't write to {}: {}", display, why),
	Ok(_) => println!("successfully wrote to {}", display),
    };
}
