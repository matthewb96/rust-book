//! # Vector Stats
//!
//! Crate based on rust book chapter 8 exercise
//!
//! > Given a list of integers, use a vector and return the
//! > median (when sorted, the value in the middle position)
//! > and mode (the value that occurs most often; a hash map
//! > will be helpful here) of the list.

use std::collections::HashMap;

/// Counts unique values and prints the 5 most common with their counts.
pub fn count_values(numbers: &Vec<i8>) {
    let mut unique_count: HashMap<i8, u32> = HashMap::new();

    for n in numbers.iter() {
        let count = unique_count.entry(*n).or_insert(0);
        *count += 1;
    }

    let mut count: Vec<_> = unique_count.iter().collect();
    count.sort_by(|a, b| a.1.cmp(b.1).reverse());

    println!("Most common");
    println!("Value | Count");
    println!("----- | -----");
    for n in count.iter().take(5) {
        println!("{:5} | {:5}", n.0, n.1);
    }
}

/// Prints min, max and median for a **sorted** vector.
pub fn stats(sorted: &Vec<i8>) {
    println!("Min    : {}", &sorted[0]);
    println!("Max    : {}", &sorted[sorted.len() - 1]);
    println!("Median : {}", &sorted[sorted.len() / 2 - 1]);
}
