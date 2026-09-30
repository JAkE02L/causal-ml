"""Polynomial regression and prediction error decomposition.

Decomposes the Mean Squared Prediction Error (MSPE) into:
- Reducible Error: MSE between true f(X) and predicted f_hat(X)
- Irreducible Error: Variance of the random noise epsilon (sigma^2)
"""

import numpy as np


def calc_mspe(y_true: np.ndarray, y_pred: np.ndarray) -> float:
    """Calculate Mean Squared Prediction Error."""
    return float(np.mean((y_true - y_pred) ** 2))


def calc_reducible_error(true_fx: np.ndarray, y_pred: np.ndarray) -> float:
    """Calculate Reducible Error (MSE between true function and prediction)."""
    return float(np.mean((true_fx - y_pred) ** 2))


def calc_irreducible_error(y_true: np.ndarray, true_fx: np.ndarray) -> float:
    """Calculate Irreducible Error (variance of the intrinsic noise)."""
    return float(np.mean((y_true - true_fx) ** 2))


def main() -> None:
    # 1. Setup simulation parameters
    n = 100
    mean = 0.0
    sigma = 8.0

    rng = np.random.default_rng(seed=42)

    # 2. Predictor variable X ~ N(0, 1)
    X = rng.normal(0.0, 1.0, size=n)

    # 3. Regression coefficients b0...b3 = 1, 2, -2, 3
    b0, b1, b2, b3 = 1.0, 2.0, -2.0, 3.0

    # 4. True data-generating function f(X)
    true_fx = b0 + b1 * X + b2 * (X**2) + b3 * (X**3)

    # 5. Random error ~ N(mean, sigma) and observed outcome Y
    random_error = rng.normal(mean, sigma, size=n)
    Y = true_fx + random_error

    # 6. Irreducible error
    irreducible_err = calc_irreducible_error(Y, true_fx)

    # 7. Fit polynomials of degrees 1 to 10 and print table
    print(f"{'Degree':<8}{'MSPE':<14}{'ReducibleError':<18}{'IrreducibleError':<18}")
    print("-" * 58)

    for degree in range(1, 11):
        coeffs = np.polyfit(X, Y, deg=degree)
        y_pred = np.polyval(coeffs, X)

        mspe = calc_mspe(Y, y_pred)
        reducible_err = calc_reducible_error(true_fx, y_pred)

        print(f"{degree:<8}{mspe:<14.4f}{reducible_err:<18.4f}{irreducible_err:<18.4f}")


if __name__ == "__main__":
    main()
