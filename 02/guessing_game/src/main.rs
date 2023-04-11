// https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html

// we need io from the standard library, very C like here
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    
    println!("Lets play a game! Guess the number:");

    let secret_numer = rand::thread_rng().gen_range(1..=100);

    // mut <- mutable variables (not readonly) as all variables in rust are immutable by default (readonly)
    let mut guess ;

    loop {
        //Need to re-shadow the old number u32 type on return
        guess = String::new();
        println!("Input a guess for the number");
        io::stdin()
            //& is a reference to the variable guess, like a pointer but with with more memory safety
            .read_line(&mut guess)
            //a result (https://doc.rust-lang.org/stable/std/result/enum.Result.html) is returned by this function,
            //the result needs to be managed. Ok is the happy path, error is not
            .expect("Failed to read line!");

        //Wowza, okay this is neat. The old variable guess (string) is being _shadowed_ by this u32 version
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_numer) {
            Ordering::Less => println!("Too small ..."),
            Ordering::Equal => {
                println!("That's the number!");
                // How we get out of the loop we _break_ out
                break;
            },
            Ordering::Greater => println!("Too large ... ")
        }

    }

}
