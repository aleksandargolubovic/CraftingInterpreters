use std::env;
use std::process;

use crafting_interpreters::Config;
use crafting_interpreters::Lox;
// use crafting_interpreters::Token;
// use crafting_interpreters::TokenType;
// mod token;
// use token::{Token, TokenType};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);

    // let third = &v[2];

    // println!("The third element is: {third}");
    // v.push(7);
    // let third = v.get(0..2);
    // let mut v = vec![1, 2, 3];
    // for i in &mut v {
    //   *i += 50;
    // }

    // enum SpredsheetCell {
    //   Int(i32),
    //   Float(f64),
    //   Text(String),
    // }

    // let row = vec![
    //   SpredsheetCell::Int(3),
    //   SpredsheetCell::Float(10.12),
    //   SpredsheetCell::Text(String::from("aaaa")),
    // ];

    // let data = "blabla";
    // let s = data.to_string();
    // let s = "initial content".to_string();
    // let s = String::from("blabla");
    // let mut s = String::from("foo");
    // s.push_str("bar");
    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("{s2}");

    // let t = Token::new(TokenType::LEFT_PAREN, "ssss".to_string(), 55);
    // println!("{t}");
    // let t = Token::new(TokenType::IDENTIFIER(String::from("ssfsfs")), "ssss".to_string(), 44);
    // println!("{t}");

    if args.len() > 2 {
      panic!("too many arguments!");
      // error
    } else if args.len() == 2 {
      let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
      });
      let mut lox = Lox::new();
      if let Err(e) = lox.run_file(config) {
        println!("Application error: {e}");
        process::exit(1);
      }
    } else {
      let mut lox = Lox::new();
      if let Err(e) = lox.run_prompt() {
        println!("Application error: {e}");
        process::exit(1);
      }
    }
}

