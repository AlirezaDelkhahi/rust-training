use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, stdin, Write};

pub fn guessing_number() {
    let mut inp = String::new();

    let mut rng = rand::thread_rng();

    let rnd_num = rng.gen_range(1..=50);
    println!("rnd num ===> {rnd_num}");

    print!("enter your guess: ");

    match io::stdout().flush(){
        Ok(_) => print!(""),
        Err(e) => print!("{e}"),
    }

    stdin().read_line(&mut inp).expect("error getting input.");

    let inp: u32 = inp.trim().parse().expect("please enter a number");

    match inp.cmp(&rnd_num) {
        Ordering::Less => println!("your guess is less than random number."),
        Ordering::Equal => println!("your guess is right."),
        Ordering::Greater => println!("your guess is higher than random number."),
    }
}
