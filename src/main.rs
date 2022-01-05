use std::io::{self, Read};
  fn main() {
    for number in io::stdin().bytes() {
        let character:char = number.unwrap() as char;
        println!("Hello {}", character);
     }
  } 