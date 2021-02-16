use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivina un número del 1 al 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Introduce tu predicción");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falló al leer el dato");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tu predicción: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muy bajo"),
            Ordering::Greater => println!("Muy alto"),
            Ordering::Equal => {
                println!("Adivinaste!");
                break;
            }
        }
    }
}