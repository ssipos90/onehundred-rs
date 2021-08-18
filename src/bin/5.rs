use std::io::{self};

fn main () {
  let mut buff = String::new();
  io::stdin().read_line(&mut buff).expect("Failed reading buffer");

  let n = buff.trim().parse::<u64>().unwrap();

  println!(
    "mno, {}.",
    (1..n + 1)
      .filter(|n| n % 3 == 0 || n % 5 == 0)
      .sum::<u64>());
}
