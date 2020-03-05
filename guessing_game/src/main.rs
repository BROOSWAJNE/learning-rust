// allows us to do `io::stdin` instead of `std::io::stdin`
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// array type definition is [T; n] where T is element type, n is length
// &'static str effectively means that the string is hardcoded into the executable and in memory
// for the lifetime of the program
const FRUITS: [&'static str; 5] = [
    "apple",
    "banana",
    "cherry",
    "pineapple",
    "watermelon",
];

fn is_a_fruit(str : &str) -> bool {
    // arrays are not iterators, "borrowing" the array with &FRUITS does work however. not sure
    // why yet, debug message says slices are iterators, which might be what we're making
    // could also apparently do FRUITS.iter()
    for fruit in &FRUITS {
        if fruit.cmp(&str) == Ordering::Equal { return true }
    }
    false
}

fn main() {
    println!("Guess the fruit!");
    println!("Possible fruits: {}", FRUITS.len());
    
    let fruit_idx = rand::thread_rng().gen_range(1, FRUITS.len());
    let answer = FRUITS[fruit_idx].to_lowercase();

    let mut guesses = 0;
    loop {
        guesses += 1;

        println!("Attempt {}! Please input your fruit:", guesses);

        // variables are immutable by default, `mut` changes that
        // ::new means that `new` is an `associated function`, aka. static method
        let mut guess = String::new();

        // read_line takes a string as argument, mutates it to be the contents entered
        io::stdin()
            // `&` indicates we're passing a `reference` (aka. pointer) to `guess`,
            // otherwise we'd be copying its value
            // `&mut` is because references are also immutable by default, otherwise `&guess` works
            .read_line(&mut guess)
            // .read_line returns a `Result` type (which is an enum), which has an `expect` method
            // crashes the program if the Result is an `Err`, ignored if it's `Ok`
            .expect("Failed to read line");

        let guess = guess.trim().to_lowercase();
        println!("You guessed: {}", guess);

        if guess.cmp(&answer) == Ordering::Equal {
            println!("You win! You can have my {}!", answer.to_uppercase());
            break;
        } else if is_a_fruit(&guess) {
            println!("Nope.");
        } else {
            println!("That's not a fruit, come on.");
        }
    }
}
