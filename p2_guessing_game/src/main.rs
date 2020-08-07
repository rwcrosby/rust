use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret = rand::thread_rng().gen_range(1,101);
    println!("Secret: {}", secret);

    loop {

        println!("Enter a guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Woops, didn't read");

        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Enter a number");
                    continue;
                }
            };
        println!("Guessed {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low"),
            Ordering::Equal => {
                    println!("Hit");
                    break;
                },
            Ordering::Greater => println!("Too high")
        }

    }

}
