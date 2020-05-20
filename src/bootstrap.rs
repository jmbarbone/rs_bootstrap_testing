use rand::Rng;

pub extern fn bootstrap(x: &Vec<f32>, _r: &i32) -> Vec<f32> {
    
    let n = x.len() as i32;
    let estimate = mean(&x) as f32;
    let mut out = vec![];

    for _ in 0..*_r {

        let samp: Vec<f32> = sample_with_replacement(x, n);
        let samp_est: f32 = mean(&samp);
        let mut samp_se: f32 = sterr(&samp);

        if samp_se == 0.0 {
            samp_se = 1.0 / (2.0 * n as f32);
        }

        out.push((samp_est - estimate) / samp_se);
    }
    out
}

pub extern fn sample_with_replacement(x_vector: &Vec<f32>, n_replacements: i32) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    let size = x_vector.len() as i32;
    let mut result:Vec<f32> = vec![];
    
    let indexes: Vec<i32> = (0..n_replacements)
        .map(|_| {
            rng.gen_range(0, size)
        }).collect();
    
    for i in 0..n_replacements {
        result.push(x_vector[indexes[i as usize] as usize]);
    }
    
    result
}

pub extern fn mean(numbers: &Vec<f32>) -> f32 {
    numbers.iter().sum::<f32>() as f32 / numbers.len() as f32
}

pub extern fn variance(numbers: &Vec<f32>) -> f32 {
    let n: usize = numbers.len();
    let estimate: f32 = mean(&numbers);
    let mut diffs: Vec<f32> = vec![];
    for i in numbers.iter() {
        let d = i - estimate;
        diffs.push(d * d);
    }
    diffs.iter().sum::<f32>() / (n - 1) as f32
}

pub extern fn stdev(numbers: &Vec<f32>) -> f32 {
    variance(&numbers).sqrt() as f32
}

pub extern fn sterr(numbers: &Vec<f32>) -> f32 {
    stdev(&numbers) as f32 / (numbers.len() as f32).sqrt() as f32
}

