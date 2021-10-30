// Bring types into scope using the `use` keyword (aka the prelude)
use std::io;


fn main() {

    // println! is a macro that prints to the console
    println!("Guess the number!");

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
}
