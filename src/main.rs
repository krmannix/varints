use std::env; 
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Whoops! {}", err);
    process::exit(1)
  });

  println!("Integer: {:?}", to_bitv_reversed(config.integ));
  
  println!("Chunks: {:?}", to_chunks(&to_bitv_reversed(config.integ)))
}

fn run(config: Config) {

}

fn to_chunks<'a>(bits: &'a Vec<bool>) -> Vec<Vec<bool>> {
  let chunks = bits.chunks(7).enumerate();
  return chunks.clone().map(|(i, chunk)| {
    let mut vec = Vec::with_capacity(8);
    for x in 0..7 {
      vec.push(
        if x < chunk.len() { chunk[x] } else { false }
      )
    }
    vec.push(i == chunks.len() - 1); // set MSB to indicate whether more bytes are included
    return vec;
  }).collect();
}


fn to_bitv_reversed(int: u32) -> Vec<bool> {
  fn to_bitv_reversed_acc(mut acc: Vec<bool>, int: u32) -> Vec<bool> {
    if int == 0 {
      acc
    } else {
      acc.push(int % 2 == 1);
      to_bitv_reversed_acc(acc, int / 2)
    }
  }

  to_bitv_reversed_acc(Vec::with_capacity(32), int)
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

