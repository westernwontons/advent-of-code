#![allow(unused_mut, unused_variables)]

use std::{collections::BTreeMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
  let filename = "dataset1.txt";
  let dataset1 = fs::read_to_string(filename)?;

  let parsed = dataset1
    .lines()
    .map(|line| {
      line
        .split("\n")
        .flat_map(|word| {
          word
            .chars()
            .map(|char| char.to_digit(2).expect("Failed to parse char to digit"))
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let mut zeroes = 0;
  let mut ones = 0;
  let mut btreemap: BTreeMap<usize, (u32, u32)> = BTreeMap::new();

  for pos in 0..=11 {
    for (index, vec) in parsed.iter().enumerate() {
      let digit_col = vec.iter().nth(pos).unwrap();

      match digit_col {
        0 => zeroes += 1,
        1 => ones += 1,
        _ => unreachable!(),
      }

      if index == parsed.len() - 1 {
        btreemap.insert(pos + 1, (zeroes, ones));
        zeroes = 0;
        ones = 0;
      }
    }
  }

  let mut gamma_rate = String::from("");
  let mut epsilon_rate = String::from("");

  for (_, (zeroes, ones)) in btreemap.into_iter() {
    if zeroes > ones {
      gamma_rate.push('0');
      epsilon_rate.push('1');
    } else if zeroes < ones {
      gamma_rate.push('1');
      epsilon_rate.push('0');
    }
  }

  let decimal_gamma_rate = usize::from_str_radix(&gamma_rate, 2)?;
  let decimal_epsilon_rate = usize::from_str_radix(&epsilon_rate, 2)?;

  println!("Gamma rate in decimal: {}", decimal_gamma_rate);
  println!("Epsilon rate in decimal: {}", decimal_epsilon_rate);

  println!(
    "Power consumption: {}",
    decimal_gamma_rate * decimal_epsilon_rate
  );

  Ok(())
}
