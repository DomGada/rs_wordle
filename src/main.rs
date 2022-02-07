extern crate termios;
use std::fs;
use rand::Rng;
use std::io;
use std::io::Read;
use std::io::Write;
use stf::collections::HashMap;
//use colored::Colorize;

use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

fn main() {
   let f = fs::read_to_string("wordle_words.txt").expect("Can't open the file bruh");

   let mut vec:Vec<String> = Vec::new();

   for line in f.lines(){
       vec.push(line.to_string());
   }
   let mut rng = rand::thread_rng();

   let guess = rng.gen_range(0..vec.len());
   println!("Guess is {}",guess);
   
   //let guess_chance = 0;
   println!("Here is your new word {}", vec[guess]);
   let mut letter_guess = 0;
   let mut guess_word = String::new();
   while letter_guess < 4{
       println!("{}", guess_word);
       println!("{} Letters used", letter_guess);
       let input_return = mod_input();
       println!("{:?} ASCII Bull shit", input_return[0]);
       if input_return[0] == 127 { 
         guess_word.pop();
         letter_guess -=1
       } 
       else{
        guess_word.push(input_return[0] as char);
        letter_guess += 1;
       }
   }

}

fn mod_input() -> [u8; 1]{
   let stdin = 0;
   let termios = Termios::from_fd(stdin).unwrap();
   let mut new_termios = termios.clone();

   new_termios.c_lflag &= !(ICANON | ECHO);
   tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
   let stdout = io::stdout();
   let mut reader = io::stdin();
   let mut buffer = [0;1];
   print!("Hit a key ");
   stdout.lock().flush().unwrap();
   reader.read_exact(&mut buffer).unwrap();
   println!("You hit : {:?}", buffer[0] as char);
   tcsetattr(stdin,TCSANOW, & termios).unwrap();
   buffer
}


