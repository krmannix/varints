mod lib;

use std::env; 
use std::process;
use lib::to_hex;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Whoops! {}", err);
    process::exit(1)
  });

  let hexv = to_hex(config.int);
  let s: String = hexv.into_iter().collect();
  println!("{}", s);
}

struct Config {
  int: u32,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() != 2 { return Err("Please enter a non-negative integer as an argument"); }

    match args[1].clone().trim().parse::<u32>() {
      Ok(int) => Ok(Config { int } ),
      Err(_) => Err(
        "Could not parse integer.\nTry entering a non-negative integer less than 4,294,967,296"
      )
    }
  }
}
  
 
