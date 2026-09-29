use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::IndexedRandom;

fn main() {
    let population = [9, 10, 11, 12, 13, 14, 15, 16];
    let num_rows = 5000;
    let num_cols = 10;

    let mut rng = StdRng::seed_from_u64(42);
    let mut samples = vec![vec![0; num_cols]; num_rows];

    for col in 0..num_cols {
        for row in samples.iter_mut() {
            row[col] = *population.choose(&mut rng).unwrap();
        }
    }

    // Predictor 1: f_1 = 10
    let predictions_f1 = vec![vec![10.0; num_cols]; num_rows];
    let mut mspe_f1_per_sample = Vec::with_capacity(num_cols);
    for col in 0..num_cols {
        let sum_sq_err: f64 = samples
            .iter()
            .zip(predictions_f1.iter())
            .map(|(row_s, row_p)| {
                let err = f64::from(row_s[col]) - row_p[col];
                err * err
            })
            .sum();
        mspe_f1_per_sample.push(sum_sq_err / (num_rows as f64));
    }
    let mspe_f1_overall: f64 = mspe_f1_per_sample.iter().sum::<f64>() / (num_cols as f64);

    // Predictor 2: f_2 = sample mean
    let sample_means: Vec<f64> = (0..num_cols)
        .map(|col| {
            let sum: f64 = samples.iter().map(|row| f64::from(row[col])).sum();
            sum / (num_rows as f64)
        })
        .collect();

    let mut predictions_f2 = vec![vec![0.0; num_cols]; num_rows];
    for row in predictions_f2.iter_mut() {
        for (col, &mean) in sample_means.iter().enumerate() {
            row[col] = mean;
        }
    }

    let mut mspe_f2_per_sample = Vec::with_capacity(num_cols);
    for col in 0..num_cols {
        let sum_sq_err: f64 = samples
            .iter()
            .zip(predictions_f2.iter())
            .map(|(row_s, row_p)| {
                let err = f64::from(row_s[col]) - row_p[col];
                err * err
            })
            .sum();
        mspe_f2_per_sample.push(sum_sq_err / (num_rows as f64));
    }
    let mspe_f2_overall: f64 = mspe_f2_per_sample.iter().sum::<f64>() / (num_cols as f64);

    println!("Predictor 1 (f_1 = 10):");
    let rounded_f1: Vec<f64> = mspe_f1_per_sample
        .iter()
        .map(|&x| (x * 10000.0).round() / 10000.0)
        .collect();
    println!("  MSPE per sample: {rounded_f1:?}");
    println!("  Overall MSPE: {mspe_f1_overall:.4}");

    println!("\nPredictor 2 (f_2 = sample mean):");
    let rounded_f2: Vec<f64> = mspe_f2_per_sample
        .iter()
        .map(|&x| (x * 10000.0).round() / 10000.0)
        .collect();
    println!("  MSPE per sample: {rounded_f2:?}");
    println!("  Overall MSPE: {mspe_f2_overall:.4}");
}
