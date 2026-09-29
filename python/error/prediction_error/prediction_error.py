import numpy as np


def main() -> None:
    population = np.arange(9, 17)
    rng = np.random.default_rng(seed=42)
    samples = rng.choice(population, size=(5000, 10), replace=True)

    # Predictor 1: f_1 = 10
    predictions_f1 = np.full_like(samples, 10, dtype=float)
    mspe_f1_per_sample = np.mean((samples - predictions_f1) ** 2, axis=0)
    mspe_f1 = np.mean((samples - predictions_f1) ** 2)

    # Predictor 2: f_2 = sample mean
    sample_means = np.mean(samples, axis=0)
    predictions_f2 = np.tile(sample_means, (samples.shape[0], 1))
    mspe_f2_per_sample = np.mean((samples - predictions_f2) ** 2, axis=0)
    mspe_f2 = np.mean((samples - predictions_f2) ** 2)

    print("Predictor 1 (f_1 = 10):")
    print("  MSPE per sample:", np.round(mspe_f1_per_sample, 4))
    print(f"  Overall MSPE: {mspe_f1:.4f}")

    print("\nPredictor 2 (f_2 = sample mean):")
    print("  MSPE per sample:", np.round(mspe_f2_per_sample, 4))
    print(f"  Overall MSPE: {mspe_f2:.4f}")


if __name__ == "__main__":
    main()
