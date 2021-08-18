use std::io::{self, Write};

fn main () {
  print!("Number: ");
  io::stdout().flush().expect("shit's not flushing.");

  let mut buff = String::new();
  io::stdin().read_line(&mut buff).expect("Failed reading buffer");

  let n = buff.trim().parse::<u64>().unwrap();

  print!("(s)um or (p)roduct? ");
  io::stdout().flush().expect("shit's not flushing.");

  let mut buff = String::new();
  io::stdin().read_line(&mut buff).expect("Failed reading buffer");

  match buff.trim() {
    "s" | "sum" => println!("sum is {}.", (1..n + 1).sum::<u64>()),
    "m" | "mul" | "p" | "product" => println!("product is {}.", (1..n + 1).product::<u64>()),
    _ => println!("you dumb? instructions unclear? dick stuck in blender?")
  };
}
