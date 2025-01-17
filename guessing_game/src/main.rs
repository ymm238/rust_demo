use std::io;

fn main() {
    println!("Guessing number game!");

    println!("Input you number!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guess number: {}", guess);
}
