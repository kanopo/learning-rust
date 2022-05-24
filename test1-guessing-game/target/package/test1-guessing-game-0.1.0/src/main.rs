use rand::prelude::*;
use std::io;
use std::cmp::Ordering::*;

fn main() {
    println!("Welcome to guess a number!!!");
    let secret: u8 = random();
    println!("The secret number is: {secret}");

    let mut remaining_chance: u8 = 10;

    let mut my_guess: u8;

    loop {
        if remaining_chance == 0 {
            print!("You loose :(");
            break;
        }

        println!("Remainging tries: {remaining_chance}");

        let mut tmp_string_to_number = String::new();

        io::stdin()
            .read_line(&mut tmp_string_to_number).expect("Error during the input!");

        my_guess = tmp_string_to_number.trim().parse().expect("Error: u8 needed");


        println!("Your guess is: {my_guess}");

        match my_guess.cmp(&secret) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                print!("You have guessed the correct number!");
                break;
            }
        }
        
        remaining_chance -= 1;
    }

}
