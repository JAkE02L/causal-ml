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


if __name__ == "__main__":
    main()
