use std::env; 
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Whoops! {}", err);
    process::exit(1)
  });

  println!("Here")

}

struct Config {
  integ: u32,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() != 2 { return Err("Please enter an integer as an argument"); }
    
    match args[1].clone().trim().parse::<u32>() {
      Ok(integ) => Ok(Config { integ } ),
      Err(_) => Err("Could not parse integer") 
    }
  } 
}

