#![allow(unused_variables, unused_mut)]

use std::{collections::BTreeMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
  // Start of Part 1
  let filename = "dataset.txt";
  let dataset = fs::read_to_string(filename)?;

  // parse the data into a nested array
  let parsed = dataset
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

  // traverse the top level array vertically
  // on each round, using only the first element of the subarrays
  // do this until
  for pos in 0..=parsed[0].len() - 1 {
    for (index, vec) in parsed.iter().enumerate() {
      // iterate over the columns one by one
      let digit_col = vec
        .iter()
        .nth(pos)
        .expect("Length of sub array is more than expected");

      match digit_col {
        0 => zeroes += 1,
        1 => ones += 1,
        // only ones and zeroes in this array, this case should never be matched
        _ => unreachable!("Not just ones and zeroes in the sub arrays, panic!"),
      }

      // reset the intermediate variables after each iteration
      // but save the results into a btreemap before that
      // (using a btreemap because order is relevant)
      if index == parsed.len() - 1 {
        btreemap.insert(pos, (zeroes, ones));
        zeroes = 0;
        ones = 0;
      }
    }
  }

  let mut gamma_rate = String::with_capacity(12);
  let mut epsilon_rate = String::with_capacity(12);

  for (_, (zeroes, ones)) in btreemap.iter() {
    if zeroes > ones {
      gamma_rate.push('0');
      epsilon_rate.push('1');
    } else if zeroes < ones {
      gamma_rate.push('1');
      epsilon_rate.push('0');
    }
  }

  // get the decimal value of the binary numbers
  let decimal_gamma_rate = usize::from_str_radix(&gamma_rate, 2)?;
  let decimal_epsilon_rate = usize::from_str_radix(&epsilon_rate, 2)?;

  println!(
    "Power consumption: {}",
    decimal_gamma_rate * decimal_epsilon_rate
  );
  // End of Part 1

  // Start of Part 2
  {
    let mut oxygen_rating = dataset.lines().collect::<Vec<_>>();
    let mut co2_rating = dataset.lines().collect::<Vec<_>>();

    for position in 0..oxygen_rating.first().unwrap().len() {
      let zeroes = dbg!(count(&oxygen_rating, '0', position));
      let ones = dbg!(count(&oxygen_rating, '1', position));

      if zeroes > ones {
        dbg!(keep_elements(&mut oxygen_rating, '1', position));
      } else if zeroes < ones {
        dbg!(keep_elements(&mut oxygen_rating, '0', position));
      } else if zeroes == ones {
        dbg!(keep_elements(&mut oxygen_rating, '0', position));
      }

      if oxygen_rating.len() == 1 {
        break;
      }
    }

    for position in 0..co2_rating.first().unwrap().len() {
      let zeroes = dbg!(count(&co2_rating, '0', position));
      let ones = dbg!(count(&co2_rating, '1', position));

      if zeroes > ones {
        dbg!(keep_elements(&mut co2_rating, '0', position));
      } else if zeroes < ones {
        dbg!(keep_elements(&mut co2_rating, '1', position));
      } else if zeroes == ones {
        dbg!(keep_elements(&mut co2_rating, '1', position));
      }

      if co2_rating.len() == 1 {
        break;
      }
    }

    dbg!(&oxygen_rating);
    dbg!(&co2_rating);

    let first = u128::from_str_radix(oxygen_rating.first().unwrap(), 2)?;
    let second = u128::from_str_radix(co2_rating.first().unwrap(), 2)?;

    dbg!(first);
    dbg!(second);

    dbg!(first * second);
  }
  // End of Part 2

  Ok(())
}

/// Count `character` of `vector` at `position
fn count(vector: &Vec<&str>, character: char, position: usize) -> usize {
  vector
    .iter()
    .filter(|&&string| string.chars().nth(position) == Some(character))
    .count()
}

/// Keep lines in `vector` where `character` matches char at `position`
fn keep_elements(vector: &mut Vec<&str>, character: char, position: usize) {
  vector.retain(|&element| element.chars().nth(position) == Some(character))
}
