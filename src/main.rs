use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, kind player!\nWelcome to Rustaceoneer!\nChoose your class!");
    
    let mut class = String::new();

    fn read_pop(mut s: &mut String) -> &mut std::string::String {
	io::stdin()
	    .read_line(&mut s)
	    .expect("Failed to read line");
   
	s.pop();

	return s;
    }

    read_pop(&mut class);

    if class == "fighter" {
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
	read_pop(&mut a);
	if a == "y" {
	    return true;
	} else {
	    return false;
	}
    }

    println!("You're in the middle of a small market, surrounded by the houses of the people who are too rich to frequent the vendors at their doorstep.\nThere are 4 vendors there, 2 of them selling clothes, 2 selling food.  They are unfamiliar to you, but by their garments you can tell\nthey are not rich.");

    if ask("Do you want to kill the guy?".to_string()) {
	println!("cyka blyat!");
    }
}
