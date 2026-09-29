use ndarray::Array2;
use ndarray_stats::CorrelationExt;
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

    // --- Method 1: Manual Pearson correlation (pure mathematical definition) ---
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

    println!("\nCorrelation matrix (Method 1: Manual calculation):");
    for row in &corr_matrix {
        println!("{row:?}");
    }

    // --- Method 2: Vectorized Pearson correlation (via ndarray & ndarray-stats) ---
    let flat: Vec<f64> = samples
        .iter()
        .flat_map(|row| row.iter().map(|&x| f64::from(x)))
        .collect();
    let arr = Array2::from_shape_vec((num_rows, num_cols), flat).unwrap();
    // Transpose so that each of the 10 samples (columns) is treated as a random variable
    let corr_ndarray = arr.t().pearson_correlation().unwrap();
    let rounded_ndarray = corr_ndarray.mapv(|x| (x * 100.0).round() / 100.0);

    println!("\nCorrelation matrix (Method 2: ndarray + ndarray-stats):");
    for row in rounded_ndarray.rows() {
        let row_vec: Vec<f64> = row.to_vec();
        println!("{row_vec:?}");
    }

    // --- 3 Estimators of Population Parameter ---
    let mut est1 = Vec::with_capacity(num_cols);
    let mut est2 = Vec::with_capacity(num_cols);
    let mut est3 = Vec::with_capacity(num_cols);

    for col in 0..num_cols {
        // Estimator 1: Sample mean
        let sum: f64 = samples.iter().map(|row| f64::from(row[col])).sum();
        let e1 = sum / (num_rows as f64);
        est1.push((e1 * 100.0).round() / 100.0);

        // Estimator 2: 0.5 * first person + 0.5 * last person
        let first = f64::from(samples[0][col]);
        let last = f64::from(samples[num_rows - 1][col]);
        let e2 = 0.5 * first + 0.5 * last;
        est2.push((e2 * 100.0).round() / 100.0);

        // Estimator 3: 0.25 * first person + 0.75 * last person
        let e3 = 0.25 * first + 0.75 * last;
        est3.push((e3 * 100.0).round() / 100.0);
    }

    let est1_mean = est1.iter().sum::<f64>() / (num_cols as f64);
    let est2_mean = est2.iter().sum::<f64>() / (num_cols as f64);
    let est3_mean = est3.iter().sum::<f64>() / (num_cols as f64);

    println!("\nEstimator 1 (Sample Mean):");
    println!("  Estimates across samples: {est1:?}");
    println!(
        "  Overall average: {:.2}",
        (est1_mean * 100.0).round() / 100.0
    );

    println!("\nEstimator 2 (0.5 * First + 0.5 * Last):");
    println!("  Estimates across samples: {est2:?}");
    println!(
        "  Overall average: {:.2}",
        (est2_mean * 100.0).round() / 100.0
    );

    println!("\nEstimator 3 (0.25 * First + 0.75 * Last):");
    println!("  Estimates across samples: {est3:?}");
    println!(
        "  Overall average: {:.2}",
        (est3_mean * 100.0).round() / 100.0
    );
}
