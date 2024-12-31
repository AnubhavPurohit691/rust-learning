use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Guess number game");
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("secretno is {}", secret);
    loop {
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("failed process");
        // println!("we got no {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("enter valid input");
                continue;
            }
        }; //shadowing
        // print!("{guess}");
        match secret.cmp(&guess) {
            Ordering::Equal => {
                println!("you won");
                break;
            }
            Ordering::Greater => println!("its big"),
            Ordering::Less => print!("its low"),
        }
    }
}
