use std::io::{self};

fn main () {
  let mut buff = String::new();
  io::stdin().read_line(&mut buff).expect("Failed reading buffer");

  let n = buff.trim().parse::<u64>().unwrap();

  // println!("mno, {}.", (1..n + 1).sum::<u64>());
  let mut x: u64 = 0;
  for a in 1..n+1 {
    x += a;
  }
  println!("mno, {}.", x);
}
