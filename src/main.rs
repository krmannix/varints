use std::env; 
use std::process;
use std::char;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Whoops! {}", err);
    process::exit(1)
  });

  run(config);
}

fn run(config: Config) {
  let bitv_reversed = to_bitv_reversed(config.integ);
  let bytev = to_bytev(&bitv_reversed);
  let hexv: Vec<char> = to_hexv(&bytev);
  let s: String = hexv.into_iter().collect();
  println!("{}", s);
}

fn to_hexv<'a>(bytes: &'a Vec<Vec<bool>>) -> Vec<char> {
  return bytes.iter().flat_map(|bitv| {
    return bitv.chunks(4).map(|bits| {
      return nibble_to_hex(bits.to_vec());
    });
  }).collect();
}

fn nibble_to_hex(nibble: Vec<bool>) -> char {
  let int_nibble: Vec<u32> = nibble.iter().map(|&bit| if bit { 1 } else { 0 }).collect();
  let num = int_nibble[0] * 1 + int_nibble[1] * 2 + int_nibble[2] * 4 + int_nibble[3] * 8;

  if num < 10 {
    return char::from_digit(num, 10).unwrap(); 
  } else {
    return match num {
      10 => 'A',
      11 => 'B',
      12 => 'C',
      13 => 'D',
      14 => 'E',
      15 => 'F',
      _  => 'X',
    }
  }
}

fn to_bytev<'a>(bits: &'a Vec<bool>) -> Vec<Vec<bool>> {
  let last_chunk_index = bits.len() / 7;
  return bits.chunks(7).enumerate().map(|(i, chunk)| {
    let mut byte = Vec::with_capacity(8);
    for x in 0..7 {
      byte.push(
        if x < chunk.len() { chunk[x] } else { false }
      )
    }
    byte.push(i != last_chunk_index); // set MSB to indicate whether more bytes are included
    byte.reverse();
    return byte;
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

