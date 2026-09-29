import numpy as np


def main() -> None:
    population = np.arange(9, 17)
    rng = np.random.default_rng(seed=42)
    samples = rng.choice(population, size=(5000, 10), replace=True)

    print(samples[:6])


if __name__ == "__main__":
    main()
