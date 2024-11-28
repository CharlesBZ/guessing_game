use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::Colorize;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //loop is used to define the simplest kind of loop supported in Rust. It runs the code inside it until the code uses break or the program exits.
    loop {
        println!("Please input your guess.");

        //Create a variable to store user's input, mut makes variable mutable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //Shadowing converting one variable to another type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //{} placeholder for variables ie. guess string
        println!("You guessed: {}", guess);

        
        /*guess.cmp(&secret_number) compares two numbers using Rust's comparison trait
        cmp() returns an Ordering enum which has three possible values: Less, Greater, or Equal
        Pattern Matching

        The code handles all three possible outcomes:
        If the guess is too low (Ordering::Less)
        If the guess is too high (Ordering::Greater)
        If the guess is correct (Ordering::Equal)
        Color Output

        .red() and .green() suggest the code is using a coloring library (likely colored)
        Wrong guesses are shown in red
        The winning message is shown in green
        Control Flow

        The break statement in the Equal arm exits the surrounding loop when the player wins
        The game continues if the guess is wrong (Less or Greater)
        A typical game interaction might look like:

        This is a common pattern in interactive games where you provide feedback to guide the player toward the correct answer. */

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}