use std::error::Error;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process;


mod token;
mod scanner;
pub use token::*;

use scanner::Scanner;

pub struct Config {
  file_path: String,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() > 2 {
      return Err("too many arguments");
    }

    let file_path = args[1].clone();

    Ok(Config {file_path})
  }
}

pub struct Lox {
  had_error: bool,
}

impl Lox {
  pub fn new() -> Self {
    Lox {
      had_error: false,
    }
  }

  pub fn run_file(&mut self, config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    self.run(contents);
    if self.had_error {
      process::exit(65);
    }

    Ok(())
  }

  pub fn run_prompt(&mut self) -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lock().lines();
    // let mut user_input = String::new();

    while let Some(line) = lines.next() {
      let last_input = line?;//.unwrap();

      if last_input.len() == 0 {
        break;
      }

      // if user_input.len() > 0 {
      //   user_input.push_str("\n");
      // };

      // user_input.push_str(&last_input);

      self.run(last_input);
      self.had_error = false;
    }

    // println!("\nSource code: \n{}", user_input);
    Ok(())
  }

  fn run(&mut self, input: String) {

    println!("With test:\n{input}");
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()
      .unwrap_or_else(|(line, message)| {
      self.error(line, message.to_string());
      process::exit(1);
    });
    println!("tokens: ");
    for t in tokens {
      println!("{t}");
    }
  }

  pub fn error(&mut self, line: u64, message: String) {
    self.report(line, String::from(""), message);
  }

  fn report(&mut self, line: u64, where_error: String, message: String) {
    eprintln!("[line{}] Error {}: {}", line, where_error, message);
    self.had_error = true;
  }

}

