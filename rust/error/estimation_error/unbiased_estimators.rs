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

    let raw_means: Vec<f64> = (0..num_cols)
        .map(|col| {
            let sum: f64 = samples.iter().map(|row| f64::from(row[col])).sum();
            sum / (num_rows as f64)
        })
        .collect();

    let std_devs: Vec<f64> = (0..num_cols)
        .map(|col| {
            let mean = raw_means[col];
            let sum_sq: f64 = samples
                .iter()
                .map(|row| {
                    let diff = f64::from(row[col]) - mean;
                    diff * diff
                })
                .sum();
            sum_sq.sqrt()
        })
        .collect();

    let mut corr_matrix = vec![vec![0.0; num_cols]; num_cols];
    for (j, row_j) in corr_matrix.iter_mut().enumerate() {
        for (k, val_jk) in row_j.iter_mut().enumerate() {
            if j == k {
                *val_jk = 1.0;
            } else {
                let mean_j = raw_means[j];
                let mean_k = raw_means[k];
                let cov: f64 = samples
                    .iter()
                    .map(|row| (f64::from(row[j]) - mean_j) * (f64::from(row[k]) - mean_k))
                    .sum();
                let r = cov / (std_devs[j] * std_devs[k]);
                *val_jk = (r * 100.0).round() / 100.0;
            }
        }
    }

    println!("Correlation matrix:");
    for row in &corr_matrix {
        println!("{row:?}");
    }
}
