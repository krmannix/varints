use std::env; 
use std::process;
use std::slice;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Whoops! {}", err);
    process::exit(1)
  });

  println!("Integer: {:?}", to_bitv(config.integ));
  
  println!("Chunks: {:?}", to_chunks(&to_bitv(config.integ)))
}

fn run(config: Config) {

}

fn to_chunks<'a>(bits: &'a Vec<bool>) -> &[bool] {
  return bits.chunks(7).next().unwrap();
}


fn to_bitv(int: u32) -> Vec<bool> {
  fn to_bitv_acc(mut acc: Vec<bool>, int: u32) -> Vec<bool> {
    if int == 0 {
      acc.reverse();
      acc
    } else {
      acc.push(int % 2 == 1);
      to_bitv_acc(acc, int / 2)
    }
  }

  to_bitv_acc(Vec::with_capacity(32), int)
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

