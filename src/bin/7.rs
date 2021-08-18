fn main () {
  let width = 5;
  println!("{}", (1..13).fold(String::from("     "), | carry, number| format!("{}{1:>2$}", carry, number, width)));
  print!("{}", (1..13)
    .map(| first | (1..13)
      .map(|snd| first * snd)
      .fold(format!("{0:>1$}", first, width), | carry, product | format!("{}{1:>2$}", carry, product, width))
    )
    .fold(String::new(), | carry, row | format!("{}{}\n", carry, row)));
}