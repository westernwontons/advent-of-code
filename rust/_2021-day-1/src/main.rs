#![allow(unused_variables)]
#![allow(unused_mut)]
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
  let filename = "dataset.txt";
  let content = fs::read_to_string(filename)?;

  let dataset_vec = content
    .lines()
    .map(|element| element.parse::<i32>().expect("Failed to parse string"))
    .collect::<Vec<_>>();

  // Start of Part 1
  {
    let mut increase = 0;

    for (index, element) in dataset_vec.iter().enumerate() {
      // prevent index out of bounds error
      if index == dataset_vec.len() - 1 {
        break;
      }

      if element < &dataset_vec[index + 1] {
        increase += 1;
      }
    }

    println!("Sonar sweep part 1: {}", increase);
  }
  // End of Part 1

  // Start of Part 2
  {
    let mut increase = 0;

    let prev_window = dataset_vec.windows(3);
    let next_window = dataset_vec.windows(3).skip(1);

    let zipped = prev_window.zip(next_window);

    for (prev, next) in zipped {
      let prev_sum = prev.iter().sum::<i32>();
      let next_sum = next.iter().sum::<i32>();

      if next_sum > prev_sum {
        increase += 1;
      }
    }

    println!("Sonar sweep part 2: {}", increase);
  }
  // End of Part 2

  Ok(())
}
