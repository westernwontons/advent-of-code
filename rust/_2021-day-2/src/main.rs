#![allow(unused)]

use std::{error::Error, fs};

fn parse_dataset<'a>(dataset: &'a String) -> Vec<Vec<&'a str>> {
  dataset
    .lines()
    .map(|line| line.split(" ").collect::<Vec<_>>())
    .collect::<Vec<_>>()
}

fn main() -> Result<(), Box<dyn Error>> {
  let filename1 = "dataset.txt";
  let dataset1 = fs::read_to_string(filename1)?;

  let parsed = parse_dataset(&dataset1);

  // Start of Part 1
  {
    // horizontal position
    let mut forward = 0;

    // depth
    let mut depth = 0;

    for vec in parsed.iter() {
      if let [instruction, value] = *vec.as_slice() {
        let value = value.parse::<i32>()?;

        match instruction {
          "forward" => forward += value,
          "up" => depth -= value,
          "down" => depth += value,
          _ => unreachable!(),
        }
      }
    }

    let final_position = depth * forward;

    println!("Final position: {}", final_position);
  }
  // End of Part 1

  // Start of Part 2
  let filename2 = "dataset2.txt";
  let dataset2 = fs::read_to_string(filename2)?;

  {
    let mut aim = 0;
    let mut depth = 0;
    let mut forward = 0;

    let parsed = parse_dataset(&dataset2);

    for vec in parsed.iter() {
      if let [instruction, value] = *vec.as_slice() {
        let value = value.parse::<i32>()?;

        match instruction {
          "down" => aim += value,
          "up" => aim -= value,
          "forward" => {
            forward += value;
            depth += aim * value;
          }
          _ => unreachable!(),
        }
      }
    }

    let final_position = forward * depth;

    println!("{}", final_position);
  }
  // End of Part 2

  Ok(())
}
