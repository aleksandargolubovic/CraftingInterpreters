use std::env;
use std::process;

use crafting_interpreters::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);


    if args.len() > 1 {
      panic!("too many arguments!");
      // error
    } else if args.len() == 2 {
      let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
      });
      if let Err(e) = crafting_interpreters::run_file(config) {
        println!("Application error: {e}");
        process::exit(1);
      }
    } else {
      crafting_interpreters::run_prompt();
    }
}

