use polyfit_rs::polyfit_rs::polyfit;
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand_distr::{Distribution, Normal};

// ============================================================================
// Error Metric Functions
// ============================================================================

/// Calculate Mean Squared Prediction Error (MSPE)
fn calc_mspe(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let sum: f64 = y_true
        .iter()
        .zip(y_pred.iter())
        .map(|(&yt, &yp)| {
            let err = yt - yp;
            err * err
        })
        .sum();
    sum / (y_true.len() as f64)
}

/// Calculate Reducible Error (MSE between true f(X) and prediction f_hat(X))
fn calc_reducible_error(true_fx: &[f64], y_pred: &[f64]) -> f64 {
    let sum: f64 = true_fx
        .iter()
        .zip(y_pred.iter())
        .map(|(&tf, &yp)| {
            let err = tf - yp;
            err * err
        })
        .sum();
    sum / (true_fx.len() as f64)
}

/// Calculate Irreducible Error (variance of intrinsic noise epsilon)
fn calc_irreducible_error(y_true: &[f64], true_fx: &[f64]) -> f64 {
    let sum: f64 = y_true
        .iter()
        .zip(true_fx.iter())
        .map(|(&yt, &tf)| {
            let err = yt - tf;
            err * err
        })
        .sum();
    sum / (y_true.len() as f64)
}

/// Evaluate fitted polynomial c_0 + c_1*x + ... + c_d*x^d
fn eval_poly(coeffs: &[f64], x: &[f64]) -> Vec<f64> {
    x.iter()
        .map(|&xi| {
            let mut val = 0.0;
            let mut power = 1.0;
            for &c in coeffs {
                val += c * power;
                power *= xi;
            }
            val
        })
        .collect()
}

// ============================================================================
// METHOD 1: Manual / Scratch Implementations (First Principles)
// ============================================================================

/// Sample from Normal(mean, sigma) via the Box-Muller transform.
///
/// Uses two uniform samples U1, U2 in (0, 1] to generate a standard normal
/// variate Z = sqrt(-2 * ln(U1)) * cos(2 * pi * U2), then scales and shifts it.
fn sample_normal(rng: &mut StdRng, mean: f64, sigma: f64) -> f64 {
    let u1: f64 = rng.random_range(1e-10..1.0);
    let u2: f64 = rng.random_range(0.0..1.0);
    let z = (-2.0_f64 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
    mean + z * sigma
}

/// Fit a polynomial of degree `degree` using Vandermonde matrix and Modified Gram-Schmidt QR.
///
/// Solves the linear least squares problem A * c ≈ y where A is the n x (degree + 1)
/// Vandermonde design matrix: A_ij = x_i^j.
fn fit_poly(x: &[f64], y: &[f64], degree: usize) -> Vec<f64> {
    let n = x.len();
    let m = degree + 1;

    // Construct Vandermonde design matrix A
    let mut a = vec![vec![0.0; m]; n];
    for (i, &xi) in x.iter().enumerate() {
        let mut power = 1.0;
        for val in &mut a[i] {
            *val = power;
            power *= xi;
        }
    }

    // Modified Gram-Schmidt QR factorization: A = Q * R
    let mut q = a;
    let mut r = vec![vec![0.0; m]; m];

    for j in 0..m {
        let mut norm_sq = 0.0;
        for row in &q {
            norm_sq += row[j] * row[j];
        }
        let norm = norm_sq.sqrt();
        r[j][j] = norm;
        if norm > 1e-12 {
            for row in &mut q {
                row[j] /= norm;
            }
        }
        for k in (j + 1)..m {
            let mut dot = 0.0;
            for row in &q {
                dot += row[j] * row[k];
            }
            r[j][k] = dot;
            for row in &mut q {
                row[k] -= dot * row[j];
            }
        }
    }

    // d = Q^T * y
    let mut d = vec![0.0; m];
    for (j, dj) in d.iter_mut().enumerate() {
        let mut dot = 0.0;
        for (row, &yi) in q.iter().zip(y.iter()) {
            dot += row[j] * yi;
        }
        *dj = dot;
    }

    // Back-substitution: solve R * c = d
    let mut c = vec![0.0; m];
    for j in (0..m).rev() {
        let mut sum = d[j];
        for k in (j + 1)..m {
            sum -= r[j][k] * c[k];
        }
        if r[j][j].abs() > 1e-12 {
            c[j] = sum / r[j][j];
        }
    }
    c
}

// ============================================================================
// METHOD 2: Crate-Based Implementations (High-Level & Idiomatic)
// ============================================================================

/// Sample from Normal(mean, sigma) using the `rand_distr` crate (Ziggurat algorithm).
fn sample_normal_crate(rng: &mut StdRng, mean: f64, sigma: f64) -> f64 {
    let normal = Normal::new(mean, sigma).expect("invalid normal distribution parameters");
    normal.sample(rng)
}

/// Fit a polynomial of degree `degree` using the `polyfit-rs` crate.
fn fit_poly_crate(x: &[f64], y: &[f64], degree: usize) -> Vec<f64> {
    polyfit(x, y, degree).expect("polynomial fit failed")
}

// ============================================================================
// Main Execution
// ============================================================================

fn main() {
    let n = 100;
    let mean = 0.0;
    let sigma = 8.0;

    let mut rng = StdRng::seed_from_u64(42);

    // Predictor variable X ~ N(0, 1) generated via sample_normal
    let mut x = Vec::with_capacity(n);
    for _ in 0..n {
        x.push(sample_normal(&mut rng, 0.0, 1.0));
    }

    // Regression coefficients b0...b3 = 1, 2, -2, 3
    let (b0, b1, b2, b3) = (1.0, 2.0, -2.0, 3.0);

    // true_fx = b0 + b1*X + b2*X^2 + b3*X^3
    let true_fx: Vec<f64> = x
        .iter()
        .map(|&xi| b0 + b1 * xi + b2 * xi.powi(2) + b3 * xi.powi(3))
        .collect();

    // randomError = N(mean, sigma)
    let mut random_error = Vec::with_capacity(n);
    for _ in 0..n {
        random_error.push(sample_normal(&mut rng, mean, sigma));
    }

    // Y = true_fx + randomError
    let y: Vec<f64> = true_fx
        .iter()
        .zip(random_error.iter())
        .map(|(&tf, &re)| tf + re)
        .collect();

    let irreducible_err = calc_irreducible_error(&y, &true_fx);

    println!("=== Method 1: Manual First-Principles Fit (QR Decomposition) ===");
    println!(
        "{:<8}{:<14}{:<18}{:<18}",
        "Degree", "MSPE", "ReducibleError", "IrreducibleError"
    );
    println!("{}", "-".repeat(58));

    for degree in 1..=10 {
        let coeffs = fit_poly(&x, &y, degree);
        let y_pred = eval_poly(&coeffs, &x);

        let mspe = calc_mspe(&y, &y_pred);
        let reducible_err = calc_reducible_error(&true_fx, &y_pred);

        println!(
            "{:<8}{:<14.4}{:<18.4}{:<18.4}",
            degree, mspe, reducible_err, irreducible_err
        );
    }

    println!("\n=== Method 2: Crate-Based Fit (`polyfit-rs`) Comparison ===");
    println!(
        "{:<8}{:<14}{:<18}{:<18}",
        "Degree", "MSPE", "ReducibleError", "IrreducibleError"
    );
    println!("{}", "-".repeat(58));

    for degree in 1..=10 {
        let coeffs_crate = fit_poly_crate(&x, &y, degree);
        let y_pred_crate = eval_poly(&coeffs_crate, &x);

        let mspe = calc_mspe(&y, &y_pred_crate);
        let reducible_err = calc_reducible_error(&true_fx, &y_pred_crate);

        println!(
            "{:<8}{:<14.4}{:<18.4}{:<18.4}",
            degree, mspe, reducible_err, irreducible_err
        );
    }

    // Demonstration of sample_normal_crate from rand_distr
    let mut crate_rng = StdRng::seed_from_u64(42);
    let sample_val = sample_normal_crate(&mut crate_rng, mean, sigma);
    println!(
        "\nDemonstration of crate sampling (`rand_distr::Normal`): sample = {:.4}",
        sample_val
    );
}
