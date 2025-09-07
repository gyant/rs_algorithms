use insertion_sort::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();
    let random_vec: Vec<i32> = (0..10000).map(|_| rng.gen_range(0..10000)).collect();

    println!("\nRunning No-Copy Insertion Sort");
    let mut list: Vec<i32> = random_vec.clone();
    measure_fn(insertion_sort, &mut list);

    let random_vec: Vec<String> = (0..10000)
        .map(|_| (0..10).map(|_| rng.sample(Alphanumeric) as char).collect())
        .collect();

    println!("\nString: Running No-Copy Insertion Sort");
    let mut list: Vec<String> = random_vec.clone();
    measure_fn(insertion_sort, &mut list);
}

fn measure_fn<T: Ord>(op: fn(&mut Vec<T>), list: &mut Vec<T>) {
    let start = Instant::now();
    op(list);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
