use std::error::Error;
use std::fs;
use std::io;
use std::io::BufRead;


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
  hadError: bool,
}

pub fn run_file(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  run(&contents);

  Ok(())
}

pub fn run_prompt() -> Result<(), Box<dyn Error>> {
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

    run(&last_input);

  }

  // println!("\nSource code: \n{}", user_input);
  Ok(())
}

fn run(input: &String) {

  println!("With test:\n{input}");
}