use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Gussing the number!");
    println!("Input your number:");

    let selected = rand::thread_rng().gen_range(1..101);

    let mut over = false;
    while !over {
        let mut gussing = String::new();
        io::stdin()
            .read_line(&mut gussing)
            .expect("failed to read line!");

        let gussing: u32 = gussing.trim().parse().expect("Please input a number!");
        println!("you guessed: {}", gussing);

        match gussing.cmp(&selected) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                over = true;
                println!("You win!")
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
