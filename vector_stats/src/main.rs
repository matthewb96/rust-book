use rand;
use vector_stats::{stats, count_values};

fn main() {
    let mut numbers: Vec<i8> = Vec::with_capacity(10_000);
    for _ in 0..numbers.capacity() {
        numbers.push(rand::random());
    }

    numbers.sort_unstable();
    
    stats(&numbers);
    count_values(&numbers);
}
