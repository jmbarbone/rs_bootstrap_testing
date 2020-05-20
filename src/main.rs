mod bootstrap;

fn main() {
    println!("Hello, world!");

    let x:Vec<f32> = vec![1.1, 2.2, 2.3, 1.8, 3.0];
    let r:i32 = 100;
    // let result = bootstrap::bootstrap(&x, &r);
    // println!("Bootstrap result is: {:?}", result);

    let numbers = vec![1.0, 2.0, 3.0, 4.0];
    println!("Mean: {}", bootstrap::mean(&numbers));
    println!("Variance: {}", bootstrap::variance(&numbers));
    println!("Standard deviation: {}", bootstrap::stdev(&numbers));
    println!("Standard error: {}", bootstrap::sterr(&numbers));
    println!("Bootstrap??: {:?}", bootstrap::bootstrap(&x, &r));
}
