use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
  let filename = "dataset.txt";
  let dataset = fs::read_to_string(filename)?;

  let parsed = dataset
    .lines()
    .map(|line| line.split(" ").collect::<Vec<_>>())
    .collect::<Vec<_>>();

  // Start of Part 1
  {
    // horizontal position
    let mut forward = 0;

    // depth
    let mut depth = 0;

    for vec in parsed.iter() {
      if let [instruction, value] = vec.as_slice() {
        let value = value.parse::<i32>()?;

        match *instruction {
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
  {}
  // End of Part 2

  Ok(())
}
