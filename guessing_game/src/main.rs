// Bring types into scope using the `use` keyword (aka the prelude)
use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {

    // println! is a macro that prints to the console
    println!("Guess the number!");

    // .. is inclusive to the lower bound but exclusive to the upper bound (kind of wild imo)
    // range(1..101) is the same as range(1..=100)
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // let creates a new immutable variable
    // (by default variables in Rust are immutable) [https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability]
    // "When a variable is immutable, once a value is bound to a name, you can’t change that value."
    // Using the mut keyword makes the variable mutable
    // In summary: "the following line has created a mutable variable that is currently bound to a new, empty instance of a String"
    let mut guess = String::new();


    io::stdin()
        // The & indicates that this argument is a reference
        // *Note: Like vars, references are immutable by default
        // "Hence, you need to write &mut guess rather than &guess to make it mutable."
        // read_line also returns a value
        // "The read_line function returns a Result<T, E> type, which is an enum with two variants, Ok and Err."
        .read_line(&mut guess)
        // Error handling
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);

    // Convert the string to an integer
    // parse() returns a Result<T, E> type, which is an enum with two variants, Ok and Err.
    // The Ok variant contains the value that was parsed.
    // The Err variant contains the error that was encountered.
    // Create a new variable named guess
    // NOTE: This is a "shadow" variable
    // Rust allows use to shadow a variable by using the same name as the variable being shadowed
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // Ordering is an enum that has three variants:
    // Less, Equal, Greater
    // The compare "cmp" function takes two arguments of the same type and compares them
    // The match expression is used to match on the result of the cmp function
    // The match is called a "Control Flow Operator" that compares a value against a pattern
    // A match expression is made up of "arms". Each arm consists of a pattern and a block of code to execute if the pattern matches.
    // (Think of arms as branches)
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
