mod bootstrap;

use std::time::Instant;

fn main() {
    println!("Hello, world!");

    let x:Vec<f32> = vec![1.1, 2.2, 2.3, 1.8, 3.0];
    let r:i32 = 15;
    // let result = bootstrap::bootstrap(&x, &r);
    // println!("Bootstrap result is: {:?}", result);

    let numbers = vec![1.0, 2.0, 3.0, 4.0];
    println!("Mean: {}", bootstrap::mean(&numbers));
    println!("Variance: {}", bootstrap::variance(&numbers));
    println!("Standard deviation: {}", bootstrap::stdev(&numbers));
    println!("Standard error: {}", bootstrap::sterr(&numbers));
    println!("Bootstrap??: {:?}", bootstrap::bootstrap(&x, &r));

    let size:usize = 10;
    println!("Sampling: {:?}", bootstrap::sample_with_replacement(&x, &size));

    let new_size:usize = 500;
    let new_r:i32 = 2000;
    let new_vec = bootstrap::sample_with_replacement(&numbers, &new_size);
    println!("New vector size: {}", new_vec.len());
    let before = Instant::now();
    let result = bootstrap::bootstrap(&new_vec, &new_r);
    println!("Result length: {}", result.len());
    println!("Result sum: {}", result.iter().sum::<f32>());
    println!("Elapsed time: {:.2?}", before.elapsed());
}
