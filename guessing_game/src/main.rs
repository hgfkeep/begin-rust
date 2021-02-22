use std::io;
fn main() {
    println!("Gussing the number!");
    println!("Input your number:");

    let mut gussing = String::new();
    io::stdin().read_line(&mut gussing).expect("failed to read line!");

    println!("you guessed: {}", gussing);
}
