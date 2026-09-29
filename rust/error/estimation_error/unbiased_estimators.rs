use rand::seq::IndexedRandom;

fn main() {
    let population = [9, 10, 11, 12, 13, 14, 15, 16];
    let num_rows = 5000;
    let num_cols = 10;

    let mut rng = rand::rng();
    let mut samples = vec![vec![0; num_cols]; num_rows];

    for col in 0..num_cols {
        for row in samples.iter_mut() {
            row[col] = *population.choose(&mut rng).unwrap();
        }
    }

    for row in samples.iter().take(6) {
        println!("{row:?}");
    }

    let mut means = Vec::with_capacity(num_cols);
    let mut variances = Vec::with_capacity(num_cols);

    for col in 0..num_cols {
        let sum: f64 = samples.iter().map(|row| f64::from(row[col])).sum();
        let mean = sum / (num_rows as f64);
        let var: f64 = samples
            .iter()
            .map(|row| {
                let diff = f64::from(row[col]) - mean;
                diff * diff
            })
            .sum::<f64>()
            / ((num_rows - 1) as f64);

        means.push((mean * 100.0).round() / 100.0);
        variances.push((var * 100.0).round() / 100.0);
    }

    println!("Sample means: {means:?}");
    println!("Sample variances: {variances:?}");
}
