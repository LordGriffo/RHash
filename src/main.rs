// A Rust program to generate a 
// random password

//Using an Randomics Functions "rand::" to Generate a 64 key Passwords
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn main() {
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
	
    println!("Generated password: {}", password);
}
