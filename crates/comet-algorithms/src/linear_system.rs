/// Solve a system of linear equations A · x = b
/// where A is a tridiagonal matrix.
/// See https://en.wikipedia.org/wiki/Tridiagonal_matrix_algorithm
/// for more information.
///
/// Returns the solutions x of the system of linear equations.
pub fn thomas_algorithm(a: &[&[f64]], b: &[f64]) -> Vec<f64> {
    let n = b.len();

    if n == 1 {
        return vec![b[0] / a[0][0]];
    }

    let mut beta = vec![0.0; n];
    let mut gamma = vec![0.0; n];
    let mut y = vec![0.0; n];

    beta[0] = a[0][0];
    gamma[0] = a[0][1] / beta[0];
    y[0] = b[0] / beta[0];

    for i in 1..n {
        let d_i = a[i][i];
        let e_i = a[i][i - 1];
        beta[i] = d_i - e_i * gamma[i - 1];

        if i < n - 1 {
            let c_i = a[i][i + 1];
            gamma[i] = c_i / beta[i]
        }

        // backward elimination
        y[i] = (b[i] - e_i * y[i - 1]) / beta[i];
    }

    let mut x = vec![0.0; n];
    x[n - 1] = y[n - 1];

    // forward elimination
    for i in (0..=n - 2).rev() {
        x[i] = y[i] - gamma[i] * x[i + 1];
    }

    x
}

#[cfg(test)]
mod tests {
    use crate::thomas_algorithm;
    use approx::RelativeEq;

    #[test]
    fn test_1x1() {
        let a = &[[2.0].as_ref()];
        let b = &[3.0];

        let x = thomas_algorithm(a, b);
        assert_eq!(x, vec![1.5]);
    }

    #[test]
    fn test_3x3() {
        let a = &[
            [3.0, 2.0, 0.0].as_ref(),
            [2.0, 4.0, 2.0].as_ref(),
            [0.0, 2.0, 5.0].as_ref(),
        ];
        let b = &[1.0, 0.0, 3.0];

        let x = thomas_algorithm(a, b);
        assert!(<[f64]>::relative_eq(
            &x,
            &[1.0, -1.0, 1.0],
            f64::EPSILON,
            f64::EPSILON
        ));
    }

    #[test]
    fn text_5x5_identity() {
        let a = &[
            [1.0, 0.0, 0.0, 0.0, 0.0].as_ref(),
            [0.0, 1.0, 0.0, 0.0, 0.0].as_ref(),
            [0.0, 0.0, 1.0, 0.0, 0.0].as_ref(),
            [0.0, 0.0, 0.0, 1.0, 0.0].as_ref(),
            [0.0, 0.0, 0.0, 0.0, 1.0].as_ref(),
        ];
        let b = &[1.0, 42.0, 17.0, -5.0, 0.1];

        let x = thomas_algorithm(a, b);
        assert!(<[f64]>::relative_eq(&x, b, f64::EPSILON, f64::EPSILON));
    }
}
