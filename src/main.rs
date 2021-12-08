use std::io;
use std::io::Write;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let secret: u8 = rng.gen_range(0..142);

    let mut num = String::new();

    loop {
        num.clear();
        print!("Entre un nombre frérot : ");
        io::stdout().flush().expect("Erreur de flushing");

        if io::stdin().read_line(&mut num).is_err() {
            eprintln!("Erreur de lecture");
            continue;
        };

        match num.trim().parse::<u8>() {
            Ok(int_result) => {
                if int_result == secret {
                    println!("Bravo");
                    break;
                } else if int_result < secret {
                    println!("Plus");
                } else {
                    println!("Moins");
                }
            }
            Err(_) => { eprintln!("Entier attendu"); }
        }
    }
}
