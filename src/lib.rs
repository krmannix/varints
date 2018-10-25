use std::char;

pub fn to_hex(int: u32) -> Vec<char> {
  let bits_reversed = int_to_bits_reversed(int);
  let bytes = bits_reversed_to_bytes(&bits_reversed);
  return bytes_to_hex(&bytes);
}

fn bytes_to_hex(bytes: &Vec<Vec<bool>>) -> Vec<char> {
  return bytes.iter().flat_map(|bitv| {
    return bitv.chunks(4).map(|bits| {
      return nibble_to_hex(bits.to_vec());
    });
  }).collect();
}

fn nibble_to_hex(nibble: Vec<bool>) -> char {
  let num = nibble.iter().enumerate().fold(0, |acc, (i, &bit)| {
    acc + if bit { 2u8.pow((nibble.len() - i - 1) as u32) } else { 0 }
  });

  if num < 10 {
    return char::from_digit(num as u32, 10).unwrap(); 
  } else {
    return (num + 55) as char;
  }
}

fn bits_reversed_to_bytes(bits_reversed: &Vec<bool>) -> Vec<Vec<bool>> {
  let last_chunk_index = bits_reversed.len() / 7;

  return bits_reversed.chunks(7).enumerate().map(|(i, chunk)| {
    let mut byte: Vec<bool> = Vec::with_capacity(8);
    for x in 0..7 {
      byte.push(if x < chunk.len() { chunk[x] } else { false })
    }
    byte.push(i != last_chunk_index); // set MSB to indicate whether more bytes are included
    byte.reverse();
    return byte.to_vec();
  }).collect();
}


fn int_to_bits_reversed(int: u32) -> Vec<bool> {
  fn int_to_bits_reversed_acc(mut acc: Vec<bool>, xint: u32) -> Vec<bool> {
    if xint == 0 {
      acc
    } else {
      acc.push(xint % 2 == 1);
      int_to_bits_reversed_acc(acc, xint / 2)
    }
  }

  let mut vec: Vec<bool> = Vec::with_capacity(32);

  if int == 0 {
    vec.push(false);
    vec
  } else {
    int_to_bits_reversed_acc(vec, int)
  }
}

#[cfg(test)]
mod tests {
  use super::to_hex;

  #[test]
  fn one() {
    assert_eq!(to_hex(1), &['0','1'])
  }

  #[test]
  fn five_digit() {
    assert_eq!(to_hex(89657), &['B','9','B','C','0','5'])
  }
}
