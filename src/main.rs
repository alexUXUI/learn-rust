use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  loop {
    println!("\n Generating random number betwen 1 - 3 🌀");

    let secret_number = rand::thread_rng().gen_range(1, 4);

    println!("\n Guess the random Number: ___ 🤔 \n");

    let mut secret_number_guess = String::new();

    io::stdin()
      .read_line(&mut secret_number_guess)
      .expect("\n Failed to read guess ❌");

    let secret_number_guess: u32 = match secret_number_guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("\n Please enter a number");
        continue;
      }
    };

    match secret_number_guess.cmp(&secret_number) {
      Ordering::Less => println!("\n Your guess is less 😩 \n"),
      Ordering::Greater => println!("\n Your guess is more 😩 \n"),
      Ordering::Equal => {
        println!("\n 💎 YOU WIN 💎 \n");
        println!("\n The secrect number is: {} 🤫", secret_number);
        break;
      }
    }
  }
}
