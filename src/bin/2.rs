use std::io::{self, Write};

fn main() {
    print!("Hi! What should we call you? ");
    if let Err(err) = io::stdout().flush() {
        eprintln!("Error flushing output {}", err)
    }

    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            let _name = name.trim();
            if _name.len() == 0 {
                println!("\nBye, hater!");
            } else {
                println!("\nHello, {}!", name.trim())
            }
        },
        Err(err) => eprintln!("Error reading your name {}", err)
    };
}