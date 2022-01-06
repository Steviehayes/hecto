use std::io::{self, Read};
  fn main() {
    for number in io::stdin().bytes() {
        let character:char = number.unwrap() as char;
        if character == 'q' {
            break; 
        println!("Hello {}", character);
     }
    } 
  }