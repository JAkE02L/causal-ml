import numpy as np


def main() -> None:
    population = np.arange(9, 17)
    rng = np.random.default_rng(seed=42)
    samples = rng.choice(population, size=(5000, 10), replace=True)

    print(samples[:6])

    means = np.round(np.mean(samples, axis=0), 2)
    variances = np.round(np.var(samples, axis=0, ddof=1), 2)

    print("Sample means:", means)
    print("Sample variances:", variances)

    corr_matrix = np.round(np.corrcoef(samples, rowvar=False), 2)
    print("Correlation matrix:")
    print(corr_matrix)

    # --- Estimator 1: Sample Mean ---
    est1 = np.mean(samples, axis=0)
    est1_mean = np.mean(est1)
    est1_var = np.var(est1, ddof=1)

    # --- Estimator 2: 0.5 * first person + 0.5 * last person ---
    est2 = 0.5 * samples[0, :] + 0.5 * samples[-1, :]
    est2_mean = np.mean(est2)
    est2_var = np.var(est2, ddof=1)

    # --- Estimator 3: 0.25 * first person + 0.75 * last person ---
    est3 = 0.25 * samples[0, :] + 0.75 * samples[-1, :]
    est3_mean = np.mean(est3)
    est3_var = np.var(est3, ddof=1)

    print("\nEstimator 1 (Sample Mean):")
    print("  Estimates across samples:", np.round(est1, 2))
    print(f"  Overall average: {est1_mean:.2f}")
    print(f"  Variance: {est1_var:.4f}")

    print("\nEstimator 2 (0.5 * First + 0.5 * Last):")
    print("  Estimates across samples:", np.round(est2, 2))
    print(f"  Overall average: {est2_mean:.2f}")
    print(f"  Variance: {est2_var:.4f}")

    print("\nEstimator 3 (0.25 * First + 0.75 * Last):")
    print("  Estimates across samples:", np.round(est3, 2))
    print(f"  Overall average: {est3_mean:.2f}")
    print(f"  Variance: {est3_var:.4f}")


if __name__ == "__main__":
    main()
