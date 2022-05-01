use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("pleasee input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too bih"),
            Ordering::Equal => {
              println!("you win");
              break;
            }
        }
    }
}
